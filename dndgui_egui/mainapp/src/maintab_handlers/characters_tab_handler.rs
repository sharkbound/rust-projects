use eframe::egui;
use eframe::egui::{Context, FontFamily, FontId, Label, RichText, Sense, TopBottomPanel, Ui};
use eframe::egui::accesskit::Role::Caption;
use eframe::egui::panel::TopBottomSide;
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
                    campaign.data_mut().add_open_character_window(character.id)
                }
            });
            ui.separator();
        }
    });

    let open_character_ids = campaign.data().open_character_windows().iter().map(|id| *id).collect::<Vec<_>>();
    for id in open_character_ids {
        let character = match campaign.find_character_info_by_id(id) {
            Some(character) => character,
            None => continue,
        };

        egui::Window::new(&character.name).resizable(true).show(_ctx, |ui| {
            if ui.button("Close").clicked() {
                campaign.data_mut().remove_open_character_window(id);
            }

            let modifiers = character.ability_scores.ability_score_modifiers();
            ui.label(format!("STR {} ({})", character.ability_scores.strength, modifiers.strength));
            ui.label(format!("DEX {} ({})", character.ability_scores.dexterity, modifiers.dexterity));
            ui.label(format!("CON {} ({})", character.ability_scores.constitution, modifiers.constitution));
            ui.label(format!("INT {} ({})", character.ability_scores.intelligence, modifiers.intelligence));
            ui.label(format!("WIS {} ({})", character.ability_scores.wisdom, modifiers.wisdom));
            ui.label(format!("CHA {} ({})", character.ability_scores.charisma, modifiers.charisma));
        });
    }
}