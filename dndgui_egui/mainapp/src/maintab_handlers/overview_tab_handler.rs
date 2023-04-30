use eframe::egui;
use eframe::egui::{Context, FontFamily, FontId, Label, RichText, Sense, Ui};
use crate::{MainApp, MainTab};
use crate::helpers::RichTestBuilder;

pub(crate) fn render_maintab_overview(_ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    let campaign = match app.campaign {
        Some(ref mut campaign) => campaign,
        None => {
            ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
            return;
        }
    };

    let dnd_font_family = || FontFamily::Name("dnd".into());

    ui.add(RichTestBuilder::new(campaign.title()).size(30f32).font_family(dnd_font_family()).build_label());
    ui.add_space(30.0);

    ui.add(RichTestBuilder::new("Characters").size(25f32).font_family(dnd_font_family()).build_label());
    ui.add_space(10.0);

    for char_id in campaign.character_ids() {
        let character = match campaign.find_character_by_id(char_id) {
            None => continue,
            Some(character) => { character }
        };

        let character_summary_label = RichTestBuilder::new(
            &format!("{} (Race: {}, Level: {})", &character.name, character.race.to_string(), character.level))
            .font_family(dnd_font_family())
            .size(20f32)
            .build_label();

        if ui.add(character_summary_label.sense(Sense::click())).clicked() {
            app.current_maintab = MainTab::Characters;
            let character_id = character.id;
            campaign.data_mut().add_open_character_window(character_id);
        }
    }
}