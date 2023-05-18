use eframe::egui;
use eframe::egui::{Context, FontFamily, FontId, Label, RichText, Sense, Ui};
use crate::{MainApp, MainTab};
use crate::helpers::{dnd_font_family, RichTestBuilder};

pub(crate) fn render_maintab_overview(_ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    let campaign = match app.campaign {
        Some(ref mut campaign) => campaign,
        None => {
            ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
            return;
        }
    };


    ui.add(RichTestBuilder::new(campaign.title()).size(30f32).font_family(dnd_font_family()).into_label());
    // ui.add_space(30.0);
    //
    // ui.add(RichTestBuilder::new("Characters").size(25f32).font_family(dnd_font_family()).build_label());
    // ui.add_space(10.0);
    //
    // for character in campaign.all_character_infos() {
    //     let character_summary_label = RichTestBuilder::new(
    //         &format!("{} (Race: {}, Level: {})", &character.name, character.race.to_string(), character.level))
    //         .font_family(dnd_font_family())
    //         .size(20f32)
    //         .build_label();
    //
    //     if ui.add(character_summary_label.sense(Sense::click())).clicked() {
    //         app.current_maintab = MainTab::Characters;
    //         campaign.data_mut().add_open_character_window(character.id);
    //     }
    // }
}