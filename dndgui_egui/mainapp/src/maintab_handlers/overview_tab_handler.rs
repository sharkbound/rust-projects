use eframe::egui::{Context, FontFamily, Ui};
use crate::MainApp;

pub(crate) fn render_maintab_overview(_ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    if app.campaign.is_none() {
        ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
        return;
    }

    let campaign = app.campaign.as_ref().unwrap();
    let dnd_font_family = || FontFamily::Name("dnd".into());

    crate::helpers::dnd_font_label(ui, campaign.title(), 30.0, dnd_font_family());
    ui.add_space(30.0);
    // ui.label(RichText::new());

    crate::helpers::dnd_font_label(ui, "Characters", 25.0, dnd_font_family());
    ui.add_space(10.0);

    for character in campaign.iter_characters() {
        crate::helpers::dnd_font_label(ui, &character.name, 20.0, dnd_font_family());
        crate::helpers::dnd_font_label(ui, &format!("Race: {}", character.race.to_string()), 15.0, dnd_font_family());
    }
}