//! Original VectorStrike mark, drawn in Rust without donor-brand image assets.

use egui::{Color32, Context, Id, Rect, TextureHandle, TextureOptions, Ui, pos2};

const SIZE: usize = 128;

fn segment_distance(x: f32, y: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let t = (((x - a.0) * dx + (y - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((x - a.0 - t * dx).powi(2) + (y - a.1 - t * dy).powi(2)).sqrt()
}

/// Original rounded slate tile and geometric V, shared by the window and UI.
pub fn icon_rgba() -> Vec<u8> {
    let mut pixels = Vec::with_capacity(SIZE * SIZE * 4);
    for row in 0..SIZE {
        for col in 0..SIZE {
            let x = (col as f32 + 0.5) / SIZE as f32;
            let y = (row as f32 + 0.5) / SIZE as f32;
            let corner_x = (x - 0.5).abs() - 0.31;
            let corner_y = (y - 0.5).abs() - 0.31;
            let outside = corner_x.max(0.0).hypot(corner_y.max(0.0)) - 0.18;
            let alpha = ((-outside * SIZE as f32).clamp(0.0, 1.0) * 255.0) as u8;
            let left = segment_distance(x, y, (0.24, 0.27), (0.50, 0.75));
            let right = segment_distance(x, y, (0.50, 0.75), (0.78, 0.27));
            let color = if left.min(right) < 0.047 { [92, 228, 209] } else { [21, 30, 52] };
            pixels.extend_from_slice(&[color[0], color[1], color[2], alpha]);
        }
    }
    pixels
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
        // Rounded corners: the corner pixel is transparent, the centre opaque.
        assert_eq!(img.pixels[0].a(), 0);
        assert_eq!(img.pixels[64 * 128 + 64].a(), 255);
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
