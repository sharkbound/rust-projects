use eframe::egui;
use eframe::egui::{Color32, Context, FontDefinitions, FontFamily, FontId, RichText};

pub struct RichTestBuilder {
    pub text: String,
    pub font_id: FontId,
    pub color: Option<Color32>,
}

impl RichTestBuilder {
    pub fn new(text: &str) -> Self {
        Self::default().text(text)
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font_id.size = size;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn font_family(mut self, font_family: FontFamily) -> Self {
        self.font_id.family = font_family;
        self
    }

    pub fn build_rich_text(self) -> RichText {
        let richtext = RichText::new(&self.text).font(self.font_id);
        match self.color {
            Some(color) => richtext.color(color),
            None => richtext,
        }
    }

    pub fn build_label(self) -> egui::Label {
        egui::Label::new(self.build_rich_text())
    }
}

impl Default for RichTestBuilder {
    fn default() -> Self {
        RichTestBuilder {
            text: Default::default(),
            font_id: Default::default(),
            color: None,
        }
    }
}

pub fn setup_fonts(ctx: &Context) {
    let mut fontdefs = FontDefinitions::default();
    // fontdefs.font_data.insert("dnd".into(), FontData::from_static(include_bytes!("../../Dalelands.ttf")));
    fontdefs.families.insert(FontFamily::Name("dnd".into()), vec!["Hack".into()]);
    ctx.set_fonts(fontdefs);
}