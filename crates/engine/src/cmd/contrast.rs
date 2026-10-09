//! Object › Check Contrast: the WCAG 2 contrast ratio between a foreground (type or a shape's fill)
//! and a background (the shape below it, or a colour), and which accessibility levels it meets.
//!
//! `object.checkContrast {foreground?, background?}`: with two objects selected the upper one is the
//! foreground; with one, it is checked against `background` (a colour, default white, the usual
//! artboard). The ratio is `(L1 + 0.05) / (L2 + 0.05)` of the colours' relative luminances, computed
//! on their display (sRGB) values.

use serde_json::{Value, json};
use splinewright_color::{Color, Paint};
use splinewright_doc::{Document, NodeId, NodeKind};

use super::*;

pub fn specs() -> Vec<CommandSpec> {
    vec![cmd!(
        query "object.checkContrast",
        "Check Contrast",
        ["Object"],
        None,
        "{foreground?: id, background?: id | colour, ids?: [id]} WCAG 2 contrast between two objects (type uses its first character's fill; the upper object is the foreground), or one object and a background colour (default white) → {ratio, foreground, background: {hex, luminance}, aa, aaLarge, aaa, aaaLarge, nonText (3:1 for graphics and UI), summary}",
        has_selection,
        check
    )]
}

/// Relative luminance of an sRGB colour (WCAG 2.x).
pub fn luminance(rgb: [f32; 3]) -> f64 {
    let lin = |c: f32| {
        let c = f64::from(c.clamp(0.0, 1.0));
        if c <= 0.039_28 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(rgb[0]) + 0.7152 * lin(rgb[1]) + 0.0722 * lin(rgb[2])
}

/// The contrast ratio of two colours, 1 to 21.
pub fn ratio(a: [f32; 3], b: [f32; 3]) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

fn hex(rgb: [f32; 3]) -> String {
    let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02x}{:02x}{:02x}", q(rgb[0]), q(rgb[1]), q(rgb[2]))
}

/// The solid colour an object shows: a shape's top fill, or the first character's fill of type.
fn solid(d: &Document, id: NodeId, cmd: &str) -> Result<Color> {
    let n = d.node(id).ok_or(EngineError::NoNode(id))?;
    let paint: Paint = match &n.kind {
        NodeKind::Text(t) => t.runs.first().map(|r| r.style.fill.clone()).unwrap_or(Paint::None),
        _ => n.appearance.fill_paint(),
    };
    paint.color().ok_or_else(|| bad(cmd, format!("object {} has no solid fill to check (gradients, patterns and no fill can't be)", id.0)))
}

/// A `#rrggbb` (or `#rgb`) colour of the `background` parameter.
pub fn parse_hex(s: &str) -> Option<[f32; 3]> {
    let h = s.trim().trim_start_matches('#');
    let h = match h.len() {
        3 => h.chars().flat_map(|c| [c, c]).collect::<String>(),
        6 => h.to_string(),
        _ => return None,
    };
    let v = u32::from_str_radix(&h, 16).ok()?;
    Some([((v >> 16) & 255) as f32 / 255.0, ((v >> 8) & 255) as f32 / 255.0, (v & 255) as f32 / 255.0])
}

/// Which of two objects is painted on top (the later one in paint order).
fn upper(d: &Document, a: NodeId, b: NodeId) -> (NodeId, NodeId) {
    let mut order = vec![];
    d.walk(|n| {
        if n.id == a || n.id == b {
            order.push(n.id);
        }
    });
    if order.first() == Some(&a) { (b, a) } else { (a, b) }
}

fn check(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "object.checkContrast";
    let st = s.doc()?;
    let d = &st.doc;
    let picked: Vec<NodeId> = ids_param(p, "ids").unwrap_or_else(|| st.selection.objects.clone());
    let fg_id = id_param(p, "foreground");
    let bg_val = p.get("background");
    let bg_id = bg_val.and_then(Value::as_u64).map(NodeId);
    let bg_color: Option<[f32; 3]> = match bg_val {
        Some(Value::String(h)) => Some(parse_hex(h).ok_or_else(|| bad(C, "`background` must be an object id or a #rrggbb colour"))?),
        Some(Value::Number(_)) | None => None,
        Some(_) => return Err(bad(C, "`background` must be an object id or a #rrggbb colour")),
    };
    let (fg, bg): (NodeId, Option<NodeId>) = match (fg_id, bg_id, picked.as_slice()) {
        (Some(f), b, _) => (f, b),
        (None, Some(b), [one]) if *one != b => (*one, Some(b)),
        (None, None, [only]) => (*only, None),
        (None, None, [a, b]) => {
            let (top, below) = upper(d, *a, *b);
            (top, Some(below))
        }
        _ => return Err(bad(C, "select one object (checked against white) or two objects, or give `foreground` and `background`")),
    };
    let fg_rgb = solid(d, fg, C)?.to_rgb();
    let bg_rgb = match (bg, bg_color) {
        (Some(b), _) => solid(d, b, C)?.to_rgb(),
        (None, Some(c)) => c,
        (None, None) => [1.0, 1.0, 1.0],
    };
    let r = ratio(fg_rgb, bg_rgb);
    let (aa, aa_large, aaa, aaa_large, non_text) = (r >= 4.5, r >= 3.0, r >= 7.0, r >= 4.5, r >= 3.0);
    let level = if aaa {
        "AAA"
    } else if aa {
        "AA"
    } else if aa_large {
        "AA for large text only"
    } else {
        "fails WCAG"
    };
    let r2 = (r * 100.0).floor() / 100.0;
    Ok(json!({
        "ratio": r2,
        "foreground": {"hex": hex(fg_rgb), "luminance": luminance(fg_rgb)},
        "background": {"hex": hex(bg_rgb), "luminance": luminance(bg_rgb)},
        "aa": aa, "aaLarge": aa_large, "aaa": aaa, "aaaLarge": aaa_large, "nonText": non_text,
        "summary": format!("{r2:.2}:1, {level}"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_extremes_and_a_known_pair() {
        assert!((ratio([0.0; 3], [1.0; 3]) - 21.0).abs() < 1e-9);
        assert!((ratio([1.0; 3], [1.0; 3]) - 1.0).abs() < 1e-9);
        // #767676 on white is the classic 4.54:1 AA grey.
        let g = 0x76 as f32 / 255.0;
        let r = ratio([g; 3], [1.0; 3]);
        assert!((r - 4.54).abs() < 0.01, "{r}");
        assert_eq!(ratio([0.2, 0.4, 0.6], [0.9; 3]), ratio([0.9; 3], [0.2, 0.4, 0.6]), "symmetric");
    }

    #[test]
    fn hex_parsing() {
        assert_eq!(parse_hex("#fff"), Some([1.0; 3]));
        assert_eq!(parse_hex("000000"), Some([0.0; 3]));
        assert_eq!(parse_hex("#12"), None);
        assert_eq!(parse_hex("#gggggg"), None);
    }

    fn session() -> Session {
        let mut s = Session::new();
        s.execute("file.new", &json!({"width": 400, "height": 300})).unwrap();
        s
    }

    fn rect(s: &mut Session, fill: &str) -> u64 {
        let r = s.execute("shape.rectangle", &json!({"x": 10, "y": 10, "width": 100, "height": 60})).unwrap();
        let id = r["id"].as_u64().unwrap();
        s.execute("paint.setFill", &json!({"color": fill})).unwrap();
        s.execute("paint.setStroke", &json!({"none": true})).unwrap();
        id
    }

    #[test]
    fn one_object_is_checked_against_white_or_a_background_colour() {
        let mut s = session();
        rect(&mut s, "#767676");
        let r = s.execute("object.checkContrast", &json!({})).unwrap();
        assert_eq!(r["ratio"], 4.54);
        assert_eq!((r["aa"].clone(), r["aaa"].clone(), r["aaLarge"].clone()), (json!(true), json!(false), json!(true)));
        assert_eq!(r["foreground"]["hex"], "#767676");
        assert_eq!(r["background"]["hex"], "#ffffff");
        let dark = s.execute("object.checkContrast", &json!({"background": "#666666"})).unwrap();
        assert!(dark["ratio"].as_f64().unwrap() < 1.5, "{dark}");
        assert_eq!(dark["aa"], false);
        assert!(s.execute("object.checkContrast", &json!({"background": "nope"})).is_err());
    }

    #[test]
    fn two_objects_use_the_upper_one_as_the_foreground() {
        let mut s = session();
        let below = rect(&mut s, "#ffffff");
        let above = rect(&mut s, "#000000");
        s.execute("select.set", &json!({"ids": [below, above]})).unwrap();
        let r = s.execute("object.checkContrast", &json!({})).unwrap();
        assert_eq!(r["ratio"], 21.0);
        assert_eq!(r["foreground"]["hex"], "#000000", "the upper object is the foreground");
        assert_eq!(r["summary"], "21.00:1, AAA");
        // Reversed order in the selection gives the same answer.
        s.execute("select.set", &json!({"ids": [above, below]})).unwrap();
        assert_eq!(s.execute("object.checkContrast", &json!({})).unwrap()["foreground"]["hex"], "#000000");
        // Explicit roles override the stacking.
        let r = s.execute("object.checkContrast", &json!({"foreground": below, "background": above})).unwrap();
        assert_eq!(r["foreground"]["hex"], "#ffffff");
    }

    #[test]
    fn type_uses_its_character_fill_and_unusable_fills_are_refused() {
        let mut s = session();
        let bg = rect(&mut s, "#003366");
        let t = s.execute("text.create", &json!({"x": 20, "y": 40, "text": "Hello", "size": 24})).unwrap()["id"].as_u64().unwrap();
        s.execute("select.set", &json!({"ids": [t]})).unwrap();
        s.execute("text.setStyle", &json!({"fill": "#ffffff"})).unwrap();
        s.execute("select.set", &json!({"ids": [bg, t]})).unwrap();
        let r = s.execute("object.checkContrast", &json!({})).unwrap();
        assert!(r["ratio"].as_f64().unwrap() > 12.0, "{r}");
        assert_eq!(r["foreground"]["hex"], "#ffffff");
        // Three objects: say which two.
        let extra = rect(&mut s, "#00ff00");
        s.execute("select.set", &json!({"ids": [bg, t, extra]})).unwrap();
        assert!(s.execute("object.checkContrast", &json!({})).is_err());
        // No fill: nothing to check.
        s.execute("select.set", &json!({"ids": [extra]})).unwrap();
        s.execute("paint.setFill", &json!({"none": true})).unwrap();
        assert!(s.execute("object.checkContrast", &json!({})).is_err());
    }
}
