//! Edit › Spelling: Check Spelling…, changing misspelled words and the custom dictionary.
//!
//! `type.checkSpelling` checks the type objects against the bundled English dictionary
//! ([`crate::spell`], SCOWL size 50) plus the custom dictionary in the preferences
//! (`customDictionary`). Offsets are bytes of an object's plain text (the runs joined), as
//! `text.editRange` takes them. `type.spell.change` applies several replacements as one undo step;
//! the Check Spelling dialog and agents both drive these commands.

use std::collections::HashSet;

use serde_json::{Value, json};
use splinewright_doc::{Document, Node, NodeId, NodeKind, TextObject};
use splinewright_text::edit;

use super::typecmd::refresh_bounds;
use super::*;
use crate::spell::Dictionary;

/// Most suggestions a check returns per word.
const MAX_SUGGESTIONS: usize = 20;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            query "type.checkSpelling",
            "Check Spelling…",
            ["Edit", "Spelling"],
            Some("Cmd+I"),
            "{ids?: [id] (the type objects, or groups holding them; default: the selected type, else every visible, unlocked type object), suggestions?: n (default 5), ignore?: [words]} check the type against the English dictionary and the custom dictionary → {checked, count, misspellings: [{id, word, start, end (bytes of the object's plain text), suggestions}]}",
            has_doc,
            check
        ),
        cmd!(
            "type.spell.change",
            "Change Misspellings",
            [],
            None,
            "{changes: [{id, start, end, to}]} replace those byte ranges of the type objects' plain text (as from type.checkSpelling) with `to`, as one undo step → {changed}",
            has_doc,
            change
        ),
        cmd!(
            "type.spell.addWord",
            "Add to Custom Dictionary",
            [],
            None,
            "{word} add a word to the custom dictionary (Preferences › Type), which the spelling check then accepts in any case → {words}",
            always,
            add_word
        ),
        cmd!(
            query "type.spell.suggest",
            "Spelling Suggestions",
            [],
            None,
            "{word, count?: n (default 5)} → {correct: bool, suggestions}",
            always,
            suggest
        ),
    ]
}

/// The custom dictionary: the words of the `customDictionary` preference, lowercased.
fn custom_words(s: &Session) -> HashSet<String> {
    s.prefs.custom_dictionary.split(|c: char| c.is_whitespace() || c == ',' || c == ';').filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

/// The visible, unlocked type objects under `n` (a locked or hidden group hides its type).
fn collect_text(n: &Node, out: &mut Vec<NodeId>) {
    if !n.visible || n.locked {
        return;
    }
    if matches!(n.kind, NodeKind::Text(_)) && !out.contains(&n.id) {
        out.push(n.id);
    }
    if let Some(ch) = n.children() {
        for c in ch {
            collect_text(c, out);
        }
    }
}

/// The type objects to check for `p`: `ids`, else the selected type, else all of it.
fn scope(d: &Document, selection: &[NodeId], p: &Value) -> Vec<NodeId> {
    let mut out = vec![];
    let roots: Vec<NodeId> = match ids_param(p, "ids") {
        Some(ids) => ids,
        None => selection.to_vec(),
    };
    for id in &roots {
        if let Some(n) = d.node(*id) {
            collect_text(n, &mut out);
        }
    }
    if out.is_empty() && ids_param(p, "ids").is_none() {
        for l in &d.layers {
            collect_text(l, &mut out);
        }
    }
    out
}

fn plain(t: &TextObject) -> String {
    t.runs.iter().map(|r| r.text.as_str()).collect()
}

fn check(s: &mut Session, p: &Value) -> Result<Value> {
    let st = s.doc()?;
    let ids = scope(&st.doc, &st.selection.objects, p);
    let mut ok = custom_words(s);
    if let Some(a) = p.get("ignore").and_then(Value::as_array) {
        ok.extend(a.iter().filter_map(Value::as_str).map(str::to_lowercase));
    }
    let max = (p.get("suggestions").and_then(Value::as_u64).unwrap_or(5) as usize).min(MAX_SUGGESTIONS);
    let dict = Dictionary::english();
    let mut out = vec![];
    for id in &ids {
        let Some(NodeKind::Text(t)) = st.doc.node(*id).map(|n| &n.kind) else { continue };
        let text = plain(t);
        for m in dict.misspellings(&text, &ok) {
            out.push(json!({"id": id.0, "word": m.word, "start": m.start, "end": m.end, "suggestions": dict.suggest(&m.word, max)}));
        }
    }
    Ok(json!({"checked": ids.len(), "count": out.len(), "misspellings": out}))
}

fn text_mut(d: &mut Document, id: NodeId) -> Option<&mut TextObject> {
    match d.node_mut(id).map(|n| &mut n.kind) {
        Some(NodeKind::Text(t)) => Some(t),
        _ => None,
    }
}

fn change(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "type.spell.change";
    let list = p.get("changes").and_then(Value::as_array).ok_or_else(|| bad(C, "missing `changes`"))?;
    let mut changes: Vec<(NodeId, usize, usize, String)> = vec![];
    for c in list {
        let id = id_param(c, "id").ok_or_else(|| bad(C, "each change needs an `id`"))?;
        let (a, b) = (c.get("start").and_then(Value::as_u64), c.get("end").and_then(Value::as_u64));
        let (Some(a), Some(b)) = (a, b) else { return Err(bad(C, "each change needs `start` and `end`")) };
        let to = str_param(c, "to").ok_or_else(|| bad(C, "each change needs `to`"))?.to_string();
        if a > b {
            return Err(bad(C, "`start` is after `end`"));
        }
        changes.push((id, a as usize, b as usize, to));
    }
    if changes.is_empty() {
        return Err(bad(C, "nothing to change"));
    }
    // Later ranges first, so earlier offsets stay valid within an object.
    changes.sort_by(|x, y| x.0.0.cmp(&y.0.0).then(y.1.cmp(&x.1)));
    for w in changes.windows(2) {
        if w[0].0 == w[1].0 && w[1].2 > w[0].1 {
            return Err(bad(C, "changes overlap"));
        }
    }
    for (id, _, end, _) in &changes {
        let st = s.doc()?;
        let Some(NodeKind::Text(t)) = st.doc.node(*id).map(|n| &n.kind) else { return Err(bad(C, format!("node {} is not text", id.0))) };
        if *end > edit::runs_len(&t.runs) {
            return Err(bad(C, format!("a range is past the end of node {}'s text", id.0)));
        }
    }
    let n = changes.len();
    s.edit("Check Spelling", |d, _| {
        let mut touched: Vec<NodeId> = vec![];
        for (id, a, b, to) in &changes {
            let t = text_mut(d, *id).ok_or(EngineError::NoNode(*id))?;
            edit::replace_range(&mut t.runs, *a, *b, to);
            if !touched.contains(id) {
                touched.push(*id);
            }
        }
        for id in touched {
            if let Some(t) = text_mut(d, id) {
                refresh_bounds(t);
            }
        }
        Ok(())
    })?;
    Ok(json!({"changed": n}))
}

fn add_word(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "type.spell.addWord";
    let word = str_param(p, "word").map(str::trim).filter(|w| !w.is_empty()).ok_or_else(|| bad(C, "missing `word`"))?;
    if word.chars().any(|c| c.is_whitespace() || c == ',' || c == ';') {
        return Err(bad(C, "give one word"));
    }
    let mut words: Vec<String> =
        s.prefs.custom_dictionary.split(|c: char| c.is_whitespace() || c == ',' || c == ';').filter(|w| !w.is_empty()).map(str::to_string).collect();
    if !words.iter().any(|w| w.eq_ignore_ascii_case(word)) {
        words.push(word.to_string());
    }
    let joined = words.join(" ");
    s.execute("prefs.set", &json!({"key": "customDictionary", "value": joined}))?;
    Ok(json!({"words": words}))
}

fn suggest(_: &mut Session, p: &Value) -> Result<Value> {
    let word = str_param(p, "word").ok_or_else(|| bad("type.spell.suggest", "missing `word`"))?;
    let max = (p.get("count").and_then(Value::as_u64).unwrap_or(5) as usize).min(MAX_SUGGESTIONS);
    let dict = Dictionary::english();
    let correct = dict.check(word, &HashSet::new());
    Ok(json!({"correct": correct, "suggestions": if correct { vec![] } else { dict.suggest(word, max) }}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_with(texts: &[&str]) -> Session {
        let mut s = Session::new();
        s.execute("file.new", &json!({"width": 400, "height": 300})).unwrap();
        for (i, t) in texts.iter().enumerate() {
            s.execute("text.create", &json!({"x": 20, "y": 40 + 40 * i, "text": t})).unwrap();
        }
        // A new object is selected; the checks below start from no selection (all type).
        s.execute("select.none", &json!({})).unwrap();
        s
    }

    #[test]
    fn finds_misspellings_with_suggestions_and_offsets() {
        let mut s = doc_with(&["The quick brwon fox", "All fine here"]);
        let r = s.execute("type.checkSpelling", &json!({})).unwrap();
        assert_eq!(r["checked"], 2);
        assert_eq!(r["count"], 1, "{r}");
        let m = &r["misspellings"][0];
        assert_eq!(m["word"], "brwon");
        assert_eq!((m["start"].as_u64(), m["end"].as_u64()), (Some(10), Some(15)));
        assert_eq!(m["suggestions"][0], "brown");
    }

    #[test]
    fn scope_follows_the_selection_and_ignores_hidden_type() {
        let mut s = doc_with(&["teh first", "teh second"]);
        let all = s.execute("type.checkSpelling", &json!({})).unwrap();
        assert_eq!(all["count"], 2);
        let id = all["misspellings"][0]["id"].as_u64().unwrap();
        s.execute("select.set", &json!({"ids": [id]})).unwrap();
        let one = s.execute("type.checkSpelling", &json!({})).unwrap();
        assert_eq!((one["checked"].as_u64(), one["count"].as_u64()), (Some(1), Some(1)));
        // Hidden type is not checked.
        s.execute("object.hide", &json!({})).unwrap();
        s.execute("select.none", &json!({})).unwrap();
        let rest = s.execute("type.checkSpelling", &json!({})).unwrap();
        assert_eq!((rest["checked"].as_u64(), rest["count"].as_u64()), (Some(1), Some(1)), "{rest}");
    }

    #[test]
    fn change_replaces_words_in_one_undo_step() {
        let mut s = doc_with(&["teh cat and teh dog"]);
        let r = s.execute("type.checkSpelling", &json!({})).unwrap();
        let ms = r["misspellings"].as_array().unwrap();
        assert_eq!(ms.len(), 2);
        let changes: Vec<Value> = ms.iter().map(|m| json!({"id": m["id"], "start": m["start"], "end": m["end"], "to": "the"})).collect();
        let undo = s.doc().unwrap().history.undo.len();
        assert_eq!(s.execute("type.spell.change", &json!({"changes": changes})).unwrap()["changed"], 2);
        assert_eq!(s.doc().unwrap().history.undo.len(), undo + 1, "one undo step");
        let id = ms[0]["id"].as_u64().unwrap();
        let t = s.execute("text.getRange", &json!({"id": id})).unwrap();
        assert_eq!(t["text"], "the cat and the dog");
        assert_eq!(s.execute("type.checkSpelling", &json!({})).unwrap()["count"], 0);
        s.execute("edit.undo", &json!({})).unwrap();
        assert_eq!(s.execute("text.getRange", &json!({"id": id})).unwrap()["text"], "teh cat and teh dog");
    }

    #[test]
    fn change_rejects_overlaps_and_bad_ranges() {
        let mut s = doc_with(&["teh cat"]);
        let id = s.execute("type.checkSpelling", &json!({})).unwrap()["misspellings"][0]["id"].clone();
        let bad = |s: &mut Session, c: Value| s.execute("type.spell.change", &json!({"changes": c})).is_err();
        assert!(bad(&mut s, json!([{"id": id, "start": 0, "end": 99, "to": "x"}])));
        assert!(bad(&mut s, json!([{"id": id, "start": 0, "end": 3, "to": "a"}, {"id": id, "start": 2, "end": 4, "to": "b"}])));
        assert!(bad(&mut s, json!([])));
        assert!(bad(&mut s, json!([{"id": id, "start": 3, "end": 1, "to": "x"}])));
    }

    #[test]
    fn custom_dictionary_words_are_accepted() {
        let mut s = doc_with(&["Splinewright draws"]);
        assert_eq!(s.execute("type.checkSpelling", &json!({})).unwrap()["count"], 1);
        assert_eq!(s.execute("type.spell.addWord", &json!({"word": "Splinewright"})).unwrap()["words"], json!(["Splinewright"]));
        assert_eq!(s.execute("type.checkSpelling", &json!({})).unwrap()["count"], 0);
        assert_eq!(s.prefs.custom_dictionary, "Splinewright");
        assert!(s.execute("type.spell.addWord", &json!({"word": "two words"})).is_err());
        let r = s.execute("type.checkSpelling", &json!({"ignore": ["draws"]})).unwrap();
        assert_eq!(r["count"], 0);
    }

    #[test]
    fn suggest_reports_correct_words() {
        let mut s = Session::new();
        let r = s.execute("type.spell.suggest", &json!({"word": "recieve"})).unwrap();
        assert_eq!(r["correct"], false);
        assert!(r["suggestions"].as_array().unwrap().iter().any(|v| v == "receive"));
        assert_eq!(s.execute("type.spell.suggest", &json!({"word": "receive"})).unwrap()["correct"], true);
    }
}
