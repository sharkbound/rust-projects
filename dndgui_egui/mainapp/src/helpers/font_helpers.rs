use eframe::egui::{FontFamily, FontId, RichText};

pub(crate) fn dnd_font_label(ui: &mut eframe::egui::Ui, text: &str, size: f32, font_family: FontFamily) {
    ui.label(RichText::new(text).font(FontId::new(size, font_family)));
}