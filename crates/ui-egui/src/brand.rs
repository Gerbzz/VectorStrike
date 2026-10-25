//! Approved VectorStrike artwork shared by the native window and app bar.

use egui::{Color32, Context, Id, Rect, TextureHandle, TextureOptions, Ui, pos2};

const SIZE: usize = 128;

/// Approved handoff artwork, resized as one full canvas without cropping or extra padding.
pub fn icon_rgba() -> Vec<u8> {
    include_bytes!("../../../assets/branding/VectorStrike-128.rgba").to_vec()
}

fn decode() -> egui::ColorImage {
    egui::ColorImage::from_rgba_unmultiplied([SIZE, SIZE], &icon_rgba())
}
/// The icon texture, uploaded on first use and kept in the context.
fn texture(ctx: &Context) -> TextureHandle {
    let id = Id::new("vectorstrike-brand-mark");
    if let Some(tex) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return tex;
    }
    let options = TextureOptions { mipmap_mode: Some(egui::TextureFilter::Linear), ..TextureOptions::LINEAR };
    let tex = ctx.load_texture("vectorstrike-brand-mark", decode(), options);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

/// Paint the mark into `r` (square).
pub fn paint_mark(ui: &Ui, r: Rect) {
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    ui.painter().image(texture(ui.ctx()).id(), r, uv, Color32::WHITE);
}

/// The mark's texture id (tests: find the painted marks among the shapes).
#[cfg(test)]
pub fn texture_id(ctx: &Context) -> egui::TextureId {
    texture(ctx).id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VectorcraftApp;

    #[test]
    fn the_icon_decodes_to_a_rounded_128_px_tile() {
        let img = decode();
        assert_eq!(img.size, [128, 128]);
        // Preserve the approved full-canvas derivative, including its 254-alpha center.
        assert_eq!(img.pixels[0].a(), 0);
        assert_eq!(img.pixels[64 * 128 + 64].a(), 254);
    }

    #[test]
    fn the_app_bar_paints_the_icon_from_one_cached_texture() {
        let ctx = Context::default();
        crate::theme::install_fonts(&ctx);
        let mut app = VectorcraftApp::new(vectorcraft_engine::Session::new(), Default::default());
        // The system title bar hides the in-app mark (the OS draws its own icon), so paint the
        // app-drawn bar here.
        app.custom_titlebar = true;
        let mut frame = || {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| crate::chrome::app_bar(&mut app, ui));
            let uploads = out.textures_delta.set.values().flat_map(|d| d.iter()).filter(|d| d.image.size() == [128, 128]).count();
            out.textures_delta.clear();
            let marks: Vec<Rect> = out
                .shapes
                .iter()
                .filter_map(|c| match &c.shape {
                    egui::Shape::Mesh(m) if m.texture_id == texture(&ctx).id() => Some(m.calc_bounds()),
                    _ => None,
                })
                .collect();
            (uploads, marks)
        };
        let (uploads, marks) = frame();
        assert_eq!(uploads, 1);
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0].size(), egui::vec2(18.0, 18.0));
        // Later frames reuse the texture.
        let (uploads, marks) = frame();
        assert_eq!((uploads, marks.len()), (0, 1));
    }
}
