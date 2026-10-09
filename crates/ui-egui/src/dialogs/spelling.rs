//! Edit › Spelling › Check Spelling…: steps through the misspelled words of the type objects
//! (`type.checkSpelling`) and offers Change, Change All, Ignore, Ignore All and Add to the custom
//! dictionary (`type.spell.addWord`). Changes go through `type.spell.change`, one undo step each.
//!
//! Fields: `items` ([{id, word, start, end, suggestions}] still to look at), `index` (the one shown),
//! `to` (the replacement text), `changed` (words changed so far) and `checked` (type objects checked).

use serde_json::{Value, json};

use super::DialogSpec;
use crate::state::Dialog;
use crate::{SplinewrightApp, widgets};

/// The dialog kind of Check Spelling.
pub const KIND: &str = "spelling";

pub(super) const SPEC: DialogSpec = DialogSpec { heading: |_| tl!("Check Spelling").into(), body, ok: None, min_width: 380.0, ..DialogSpec::FORM };

/// Open Check Spelling on the selected type (or all of it), as `type.checkSpelling {}` finds it.
pub fn open(app: &mut SplinewrightApp) -> Result<Value, String> {
    let r = app.session.execute("type.checkSpelling", &json!({"suggestions": 8})).map_err(|e| e.to_string())?;
    let mut d = Dialog::new(KIND, json!({"items": r["misspellings"], "index": 0, "changed": 0, "checked": r["checked"]}));
    sync_to(&mut d);
    app.ui.dialog = Some(d);
    Ok(Value::Null)
}

fn items(d: &Dialog) -> Vec<Value> {
    d.fields.get("items").and_then(Value::as_array).cloned().unwrap_or_default()
}

fn index(d: &Dialog) -> usize {
    d.f64("index", 0.0).max(0.0) as usize
}

/// `to` becomes the first suggestion for the word shown (or the word itself without one).
fn sync_to(d: &mut Dialog) {
    let items = items(d);
    let to = items
        .get(index(d))
        .map(|it| it["suggestions"].get(0).and_then(Value::as_str).or_else(|| it["word"].as_str()).unwrap_or("").to_string())
        .unwrap_or_default();
    d.fields.insert("to".into(), json!(to));
}

/// What the buttons asked for this frame.
enum Act {
    Ignore,
    IgnoreAll,
    Change { all: bool },
    Add,
}

fn body(app: &mut SplinewrightApp, ui: &mut egui::Ui, d: &mut Dialog) -> bool {
    let list = items(d);
    let idx = index(d);
    let Some(cur) = list.get(idx).cloned() else {
        let (changed, checked) = (d.f64("changed", 0.0) as u64, d.f64("checked", 0.0) as u64);
        ui.label(if checked == 0 { tl!("There is no type to check.") } else { tl!("The spelling check is complete.") });
        if changed > 0 {
            ui.label(crate::i18n::fmt(tl!("{n} words changed."), &[("n", &changed.to_string())]));
        }
        return false;
    };
    let word = cur["word"].as_str().unwrap_or("").to_string();
    widgets::subheader(ui, tl!("Not in Dictionary"));
    ui.label(egui::RichText::new(&word).font(crate::theme::semibold(16.0)));
    ui.add_space(6.0);
    widgets::subheader(ui, tl!("Change To"));
    let to = d.str("to");
    if let Some(v) = widgets::text_field(ui, "spelling-to", Some(&to), 340.0, 1) {
        d.fields.insert("to".into(), json!(v));
    }
    ui.add_space(4.0);
    let suggestions: Vec<String> =
        cur["suggestions"].as_array().map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
    if suggestions.is_empty() {
        widgets::dim_label(ui, tl!("No suggestions."));
    }
    for s in &suggestions {
        if ui.selectable_label(d.str("to") == *s, s).clicked() {
            d.fields.insert("to".into(), json!(s));
        }
    }
    ui.add_space(8.0);
    let mut act = None;
    ui.horizontal_wrapped(|ui| {
        if widgets::secondary_button(ui, tl!("Ignore")).clicked() {
            act = Some(Act::Ignore);
        }
        if widgets::secondary_button(ui, tl!("Ignore All")).clicked() {
            act = Some(Act::IgnoreAll);
        }
        let can_change = !d.str("to").is_empty() && d.str("to") != word;
        ui.add_enabled_ui(can_change, |ui| {
            if widgets::secondary_button(ui, tl!("Change")).clicked() {
                act = Some(Act::Change { all: false });
            }
            if widgets::secondary_button(ui, tl!("Change All")).clicked() {
                act = Some(Act::Change { all: true });
            }
        });
        if widgets::secondary_button(ui, tl!("Add")).on_hover_text(tl!("Add to the custom dictionary")).clicked() {
            act = Some(Act::Add);
        }
    });
    ui.add_space(6.0);
    widgets::dim_label(ui, &crate::i18n::fmt(tl!("{n} of {total}"), &[("n", &(idx + 1).to_string()), ("total", &list.len().to_string())]));
    if let Some(a) = act {
        apply(app, d, a, &word);
    }
    false
}

/// Keep the items for which `keep` holds; the index follows the word that was shown.
fn retain(d: &mut Dialog, keep: impl Fn(&Value) -> bool) {
    let list = items(d);
    let idx = index(d);
    let new_idx = list.iter().take(idx).filter(|it| keep(it)).count();
    let kept: Vec<Value> = list.into_iter().filter(|it| keep(it)).collect();
    d.fields.insert("items".into(), Value::Array(kept));
    d.fields.insert("index".into(), json!(new_idx));
    sync_to(d);
}

fn same_word(a: &Value, w: &str) -> bool {
    a["word"].as_str().is_some_and(|x| x.to_lowercase() == w.to_lowercase())
}

fn apply(app: &mut SplinewrightApp, d: &mut Dialog, act: Act, word: &str) {
    match act {
        Act::Ignore => {
            d.fields.insert("index".into(), json!(index(d) + 1));
            sync_to(d);
        }
        Act::IgnoreAll => retain(d, |it| !same_word(it, word)),
        Act::Add => {
            if let Err(e) = app.run("type.spell.addWord", json!({"word": word})) {
                app.status(e);
                return;
            }
            retain(d, |it| !same_word(it, word));
        }
        Act::Change { all } => change(app, d, all, word),
    }
}

fn change(app: &mut SplinewrightApp, d: &mut Dialog, all: bool, word: &str) {
    let to = d.str("to");
    let list = items(d);
    let idx = index(d);
    // The words to change (by position in `list`).
    let chosen: Vec<usize> = if all { (0..list.len()).filter(|i| list[*i]["word"].as_str() == Some(word)).collect() } else { vec![idx] };
    let changes: Vec<Value> =
        chosen.iter().map(|i| json!({"id": list[*i]["id"], "start": list[*i]["start"], "end": list[*i]["end"], "to": to})).collect();
    if let Err(e) = app.run("type.spell.change", json!({ "changes": changes })) {
        app.status(e);
        return;
    }
    let delta = |it: &Value| to.len() as i64 - (it["end"].as_i64().unwrap_or(0) - it["start"].as_i64().unwrap_or(0));
    // The rest of the text moved: shift the offsets of the words after each change in the same object.
    let mut rest: Vec<Value> = vec![];
    for (i, it) in list.iter().enumerate() {
        if chosen.contains(&i) {
            continue;
        }
        let shift: i64 = chosen
            .iter()
            .filter(|c| list[**c]["id"] == it["id"] && list[**c]["end"].as_i64() <= it["start"].as_i64())
            .map(|c| delta(&list[*c]))
            .sum();
        let mut it = it.clone();
        it["start"] = json!(it["start"].as_i64().unwrap_or(0) + shift);
        it["end"] = json!(it["end"].as_i64().unwrap_or(0) + shift);
        rest.push(it);
    }
    let new_idx = (0..idx).filter(|i| !chosen.contains(i)).count();
    d.fields.insert("items".into(), Value::Array(rest));
    d.fields.insert("index".into(), json!(new_idx));
    let changed = d.f64("changed", 0.0) as u64 + chosen.len() as u64;
    d.fields.insert("changed".into(), json!(changed));
    sync_to(d);
}

#[cfg(test)]
mod tests {
    use super::*;
    use splinewright_engine::Session;

    fn frame(app: &mut SplinewrightApp) -> String {
        crate::tests_labels::painted_text(app, |app, ui| super::super::show(app, ui.ctx()))
    }

    fn app_with(texts: &[&str]) -> SplinewrightApp {
        let mut app = SplinewrightApp::new(Session::new(), Default::default());
        app.run("file.new", json!({"width": 400, "height": 300})).unwrap();
        for (i, t) in texts.iter().enumerate() {
            app.run("text.create", json!({"x": 20, "y": 40 + 40 * i, "text": t})).unwrap();
        }
        app.run("select.none", json!({})).unwrap();
        app
    }

    fn text_of(app: &mut SplinewrightApp, i: usize) -> String {
        let ids: Vec<u64> = {
            let st = app.session.doc().unwrap();
            let mut v = vec![];
            st.doc.walk(|n| {
                if matches!(n.kind, splinewright_engine::doc::NodeKind::Text(_)) {
                    v.push(n.id.0);
                }
            });
            v
        };
        app.run("text.getRange", json!({"id": ids[i]})).unwrap()["text"].as_str().unwrap().to_string()
    }

    #[test]
    fn the_menu_opens_the_dialog_and_change_all_fixes_every_occurrence_in_one_step() {
        let mut app = app_with(&["teh cat and teh dog", "seperate teh"]);
        crate::menus::invoke(&mut app, "type.checkSpelling", json!({}));
        let d = app.ui.dialog.as_ref().expect("the Check Spelling dialog");
        assert_eq!(d.kind, KIND);
        assert_eq!(items(d).len(), 4);
        let text = frame(&mut app);
        for label in ["Check Spelling", "Not in Dictionary", "Change To", "Ignore", "Ignore All", "Change", "Change All", "Add"] {
            assert!(text.contains(label), "{label} in {text}");
        }
        assert_eq!(app.ui.dialog.as_ref().unwrap().str("to"), "the", "the first suggestion is preselected");
        let undo = app.session.doc().unwrap().history.undo.len();
        let mut d = app.ui.dialog.take().unwrap();
        change(&mut app, &mut d, true, "teh");
        assert_eq!(app.session.doc().unwrap().history.undo.len(), undo + 1, "Change All is one undo step");
        assert_eq!(items(&d).len(), 1, "only 'seperate' is left");
        assert_eq!(text_of(&mut app, 0), "the cat and the dog");
        assert_eq!(text_of(&mut app, 1), "seperate the");
        assert_eq!(d.f64("changed", 0.0), 3.0);
        // Offsets of the word left after the change still point at it.
        let it = &items(&d)[0];
        assert_eq!(it["word"], "seperate");
        assert_eq!((it["start"].as_i64(), it["end"].as_i64()), (Some(0), Some(8)));
    }

    #[test]
    fn change_shifts_the_later_words_of_the_same_object() {
        let mut app = app_with(&["wrold and teh"]);
        let mut d = {
            crate::menus::invoke(&mut app, "type.checkSpelling", json!({}));
            app.ui.dialog.take().unwrap()
        };
        d.fields.insert("to".into(), json!("a much longer word"));
        change(&mut app, &mut d, false, "wrold");
        let rest = items(&d);
        assert_eq!(rest.len(), 1);
        // "teh" moved by the difference in length and can now be changed in the right place.
        d.fields.insert("to".into(), json!("the"));
        change(&mut app, &mut d, false, "teh");
        assert_eq!(text_of(&mut app, 0), "a much longer word and the");
    }

    #[test]
    fn ignore_all_and_add_drop_every_match_and_a_clean_document_says_so() {
        let mut app = app_with(&["teh teh Teh wrold"]);
        crate::menus::invoke(&mut app, "type.checkSpelling", json!({}));
        let mut d = app.ui.dialog.take().unwrap();
        assert_eq!(items(&d).len(), 4);
        apply(&mut app, &mut d, Act::IgnoreAll, "teh");
        assert_eq!(items(&d).len(), 1);
        assert_eq!(items(&d)[0]["word"], "wrold");
        apply(&mut app, &mut d, Act::Add, "wrold");
        assert!(items(&d).is_empty());
        assert_eq!(app.session.prefs.custom_dictionary, "wrold");
        app.ui.dialog = Some(d);
        assert!(frame(&mut app).contains("The spelling check is complete."));
    }

    #[test]
    fn a_document_without_type_says_so() {
        let mut app = app_with(&[]);
        crate::menus::invoke(&mut app, "type.checkSpelling", json!({}));
        assert!(frame(&mut app).contains("There is no type to check."));
    }
}
