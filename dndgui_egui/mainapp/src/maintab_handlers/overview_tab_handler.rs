use eframe::egui::{Context, FontFamily, FontId, Label, RichText, Sense, Ui};
use crate::{MainApp, MainTab};

pub(crate) fn render_maintab_overview(_ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    if app.campaign.is_none() {
        ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
        return;
    }

    let campaign = app.campaign.as_ref().unwrap();
    let dnd_font_family = || FontFamily::Name("dnd".into());

    crate::helpers::dnd_font_label(ui, campaign.title(), 30.0, dnd_font_family());
    ui.add_space(30.0);

    crate::helpers::dnd_font_label(ui, "Characters", 25.0, dnd_font_family());
    ui.add_space(10.0);

    for character in campaign.iter_characters() {
        let character_summary = RichText::new(
            format!("{} (Race: {}, Level: {})", &character.name, character.race.to_string(), character.level))
            .font(FontId::new(20.0, dnd_font_family()));

        if ui.add(Label::new(character_summary).sense(Sense::click().union(Sense::hover()))).clicked() {
            app.current_maintab = MainTab::Characters;
        }
        // ui.label(RichText::new(character_summary).font(FontId::new(20.0, dnd_font_family())));
        // crate::helpers::dnd_font_label(ui, &character_summary, 20.0, dnd_font_family());
    }
}