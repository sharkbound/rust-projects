use eframe::egui;
use eframe::egui::{Context, FontFamily, FontId, Label, RichText, Sense, Ui};
use crate::{MainApp, MainTab};
use crate::helpers::RichTestBuilder;

pub(crate) fn render_maintab_characters(_ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    let campaign = match app.campaign {
        Some(ref mut campaign) => campaign,
        None => {
            ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
            return;
        }
    };

    egui::Window::new("Character List").show(_ctx, |ui| {
        let character_infos = campaign.all_character_infos();
        if character_infos.is_empty() {
            ui.label("There are no characters to show here!");
            return;
        }

        for character in character_infos {
            ui.horizontal(|ui| {
                if ui.add(RichTestBuilder::new(&character.name).size(20f32).build_label().sense(Sense::click())).clicked() {
                    println!("{} was clicked", character.name);
                }
            });
            ui.separator();
        }
    });
}