//! Upstream source and project links for attribution.

use egui::Ui;

use crate::theme::Tokens;
use crate::{VectorcraftApp, icons};

/// One link row: icon, label, opens the command's URL.
fn link(app: &mut VectorcraftApp, ui: &mut Ui, icon: &str, label: &str, cmd: &str, url: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        icons::icon(ui, icon, 16.0, t.icon);
        let r = ui.add(egui::Button::new(egui::RichText::new(label).color(t.accent).size(13.0)).frame(false)).on_hover_text(url);
        if r.clicked() {
            app.open_link(cmd);
        }
    });
}

/// Website, project, and source links for attribution.
pub fn links(app: &mut VectorcraftApp, ui: &mut Ui) {
    let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
    let s = |k: &str| l[k].as_str().unwrap_or("").to_string();
    link(app, ui, "globe", tl!("ArtCraft upstream website"), "help.website", &s("website"));
    link(app, ui, "external-link", tl!("VectorCraft upstream project"), "help.appPage", &s("appPage"));
    link(app, ui, "git-branch", tl!("VectorCraft upstream source"), "help.github", &s("github"));
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
        let mut app = VectorcraftApp::new(vectorcraft_engine::Session::new(), services);
        for id in ["help.website", "help.appPage", "help.github"] {
            crate::menus::invoke(&mut app, id, serde_json::json!({}));
        }
        assert_eq!(
            *opened.lock().unwrap(),
            ["https://getartcraft.com", "https://getartcraft.com/apps/vectorcraft", "https://github.com/storytold/vectorcraft"]
        );
        // The control channel / MCP path returns the URL without opening a browser.
        let v = app.run("help.github", serde_json::json!({})).unwrap();
        assert_eq!(v["url"], "https://github.com/storytold/vectorcraft");
        assert_eq!(opened.lock().unwrap().len(), 3);
        let ctx = egui::Context::default();
        crate::theme::install_fonts(&ctx);
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| links(&mut app, ui));
        out.textures_delta.clear();
    }
}
