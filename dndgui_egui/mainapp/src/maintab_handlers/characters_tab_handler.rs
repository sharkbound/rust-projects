use eframe::egui;
use eframe::egui::{Context, Sense, TopBottomPanel, Ui};
use dndlib::AbilityScores;
use crate::{MainApp};
use crate::helpers::RichTestBuilder;

fn populate_ability_scores_collapsable(ui: &mut Ui, ability_scores: AbilityScores) {
    let ability_score_modifiers = ability_scores.ability_score_modifiers();

    // first row of ability scores: STR, DEX, CON
    ui.collapsing(RichTestBuilder::new("Ability Scores").size(17.0).into_rich_text(), |ui| {
        let number_sign_str = |val: i32| if val >= 0 { "+" } else { "-" };
        ui.horizontal(|ui| {
            let fontsize = 14.0;
            let strength_mod = ability_score_modifiers.strength;
            ui.add(RichTestBuilder::new(
                &format!("STR {}\n{}{}", ability_scores.strength, number_sign_str(strength_mod), strength_mod)).size(fontsize).into_label()
            );

            ui.separator();

            let dexterity_mod = ability_score_modifiers.dexterity;
            ui.add(RichTestBuilder::new(
                &format!("DEX {}\n{}{}", ability_scores.dexterity, number_sign_str(dexterity_mod), dexterity_mod)).size(fontsize).into_label()
            );

            ui.separator();

            let constitution_mod = ability_score_modifiers.constitution;
            ui.add(RichTestBuilder::new(
                &format!("CON {}\n{}{}", ability_scores.constitution, number_sign_str(constitution_mod), constitution_mod)).size(fontsize).into_label()
            );
            ui.separator();
        });

        ui.separator();

        // second row of ability scores: INT, WIS, CHA
        ui.horizontal(|ui| {
            let fontsize = 14.0;
            let intelligence_mod = ability_score_modifiers.intelligence;
            ui.add(RichTestBuilder::new(
                &format!("INT {}\n{}{}", ability_scores.intelligence, number_sign_str(intelligence_mod), intelligence_mod)).size(fontsize).into_label()
            );

            ui.separator();

            let wisdom_mod = ability_score_modifiers.wisdom;
            ui.add(RichTestBuilder::new(
                &format!("WIS {}\n{}{}", ability_scores.wisdom, number_sign_str(wisdom_mod), wisdom_mod)).size(fontsize).into_label()
            );

            ui.separator();

            let charisma_mod = ability_score_modifiers.charisma;
            ui.add(RichTestBuilder::new(
                &format!("CHA {}\n{}{}", ability_scores.charisma, number_sign_str(charisma_mod), charisma_mod)).size(fontsize).into_label()
            );

            ui.separator();
        });
    });
}

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
                populate_ability_scores_collapsable(ui, ability_scores);
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