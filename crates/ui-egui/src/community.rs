//! Project links: the GitHub button (header, About) and the link list. Icons are Lucide's (ISC).
//! The list ends with the credit this fork owes the upstream VectorCraft project (see `NOTICE`).

use egui::{Color32, CornerRadius, Sense, Ui, vec2};

use crate::theme::{self, Tokens};
use crate::{SplinewrightApp, icons};

/// The GitHub button's label, height and width.
fn github_layout(ui: &Ui, large: bool) -> (std::sync::Arc<egui::Galley>, f32, f32) {
    let (h, font, label) = if large { (36.0, theme::semibold(14.0), tl!("Splinewright on GitHub")) } else { (24.0, theme::semibold(12.0), "GitHub") };
    let galley = ui.painter().layout_no_wrap(label.to_string(), font, Color32::WHITE);
    let w = galley.size().x + h * 0.55 + h * 0.9;
    (galley, h, w)
}

/// The GitHub button's width (the app bar drops it when space runs out).
pub fn github_width(ui: &Ui, large: bool) -> f32 {
    github_layout(ui, large).2
}

/// A GitHub pill button: compact for the header, large for About.
pub fn github_button(app: &mut SplinewrightApp, ui: &mut Ui, large: bool) -> egui::Response {
    let (galley, h, w) = github_layout(ui, large);
    let icon = h * 0.55;
    let (r, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
    let base = Color32::from_rgb(0x2b, 0x31, 0x37);
    let fill = if resp.hovered() { base.gamma_multiply(1.25) } else { base };
    ui.painter().rect_filled(r, CornerRadius::same((h / 2.0) as u8), fill);
    let ir = egui::Rect::from_center_size(r.left_center() + vec2(h * 0.35 + icon / 2.0, 0.0), vec2(icon, icon));
    icons::paint(ui, "git-branch", ir, Color32::WHITE);
    ui.painter().galley(egui::pos2(ir.right() + h * 0.2, r.center().y - galley.size().y / 2.0), galley, Color32::WHITE);
    let resp = resp.on_hover_text(tl!("Splinewright on GitHub"));
    if resp.clicked() {
        app.open_link("help.github");
    }
    resp
}

/// One link row: icon, label, opens the command's URL.
fn link(app: &mut SplinewrightApp, ui: &mut Ui, icon: &str, label: &str, cmd: &str, url: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        icons::icon(ui, icon, 16.0, t.icon);
        let r = ui.add(egui::Button::new(egui::RichText::new(label).color(t.accent).size(13.0)).frame(false)).on_hover_text(url);
        if r.clicked() {
            app.open_link(cmd);
        }
    });
}

/// The website, GitHub, issue tracker and upstream-credit links.
pub fn links(app: &mut SplinewrightApp, ui: &mut Ui) {
    let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
    let s = |k: &str| l[k].as_str().unwrap_or("").to_string();
    link(app, ui, "globe", tl!("Splinewright Website"), "help.website", &s("website"));
    link(app, ui, "git-branch", tl!("Splinewright on GitHub"), "help.github", &s("github"));
    link(app, ui, "external-link", tl!("Report an Issue…"), "help.reportIssue", &s("issues"));
    link(app, ui, "external-link", tl!("Based on VectorCraft…"), "help.upstream", &s("upstream"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_clicks_open_the_right_urls() {
        use std::sync::{Arc, Mutex};
        let opened = Arc::new(Mutex::new(vec![]));
        let o = opened.clone();
        let services = crate::Services { open_url: Some(Box::new(move |u: &str| o.lock().unwrap().push(u.to_string()))), ..Default::default() };
        let mut app = SplinewrightApp::new(splinewright_engine::Session::new(), services);
        for id in ["help.website", "help.github", "help.upstream", "help.reportIssue"] {
            crate::menus::invoke(&mut app, id, serde_json::json!({}));
        }
        assert_eq!(
            *opened.lock().unwrap(),
            [
                "https://www.jonmcgee.online/apps/splinewright/",
                "https://github.com/game7ballJT/splinewright",
                "https://github.com/storytold/vectorcraft",
                "https://github.com/game7ballJT/splinewright/issues"
            ]
        );
        // The control channel / MCP path returns the URL without opening a browser.
        let v = app.run("help.github", serde_json::json!({})).unwrap();
        assert_eq!(v["url"], "https://github.com/game7ballJT/splinewright");
        assert_eq!(opened.lock().unwrap().len(), 4);
        let ctx = egui::Context::default();
        theme::install_fonts(&ctx);
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| links(&mut app, ui));
        out.textures_delta.clear();
    }
}
