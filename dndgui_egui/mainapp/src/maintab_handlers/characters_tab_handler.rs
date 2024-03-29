use eframe::egui;
use eframe::egui::{Context, Grid, Sense, TopBottomPanel, Ui};
use uuid::Uuid;
use dndlib::AbilityScores;
use crate::{MainApp};
use crate::helpers::{RichTestBuilder};
use crate::maintab_handlers::character_tab_handlers::render_ability_scores::{render_ability_score_stat, render_ability_scores_collapsable};



pub(crate) fn render_maintab_characters(parent_ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    let campaign = match app.campaign {
        Some(ref mut campaign) => campaign,
        None => {
            ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
            return;
        }
    };

    egui::Window::new("Character List").resizable(true).show(parent_ctx, |ui| {
        let character_infos = campaign.all_character_infos();
        if character_infos.is_empty() {
            ui.label("There are no characters to show here!");
            return;
        }

        for character in character_infos {
            ui.horizontal(|ui| {
                if ui.add(RichTestBuilder::new(&character.name).size(20f32).into_label().sense(Sense::click())).clicked() {
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

        let window = egui::Window::new(&character.name).resizable(true);
        window.show(parent_ctx, |ui| {
            TopBottomPanel::top(format!("character_{}_menu_bar", character.id)).show_inside(ui, |ui| {
                if ui.button("Close").clicked() {
                    campaign.data_mut().remove_open_character_window(id);
                }
            });

            let ability_scores = character.ability_scores;
            let skills = character.skill_modifiers;
            let saving_throws = character.saving_throws;

            ui.vertical(|ui| {
                render_ability_scores_collapsable(ui, &ability_scores, character.id);
            });

            // let modifiers = character.ability_scores.ability_score_modifiers();
            // ui.label(format!("STR {} ({})", character.ability_scores.strength, modifiers.strength));
            // ui.label(format!("DEX {} ({})", character.ability_scores.dexterity, modifiers.dexterity));
            // ui.label(format!("CON {} ({})", character.ability_scores.constitution, modifiers.constitution));
            // ui.label(format!("INT {} ({})", character.ability_scores.intelligence, modifiers.intelligence));
            // ui.label(format!("WIS {} ({})", character.ability_scores.wisdom, modifiers.wisdom));
            // ui.label(format!("CHA {} ({})", character.ability_scores.charisma, modifiers.charisma));
        });
    }
}