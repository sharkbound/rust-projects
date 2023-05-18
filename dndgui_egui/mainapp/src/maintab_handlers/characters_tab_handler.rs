use eframe::egui;
use eframe::egui::{Context, Direction, Grid, Layout, Sense, TopBottomPanel, Ui};
use uuid::Uuid;
use dndlib::AbilityScores;
use crate::{MainApp};
use crate::helpers::{dnd_font_family, RichTestBuilder};

fn number_sign(val: i32) -> &'static str {
    if val >= 0 { "+" } else { "-" }
}

// todo: grouping and centering stats and modifiers
fn add_stat_with_layout(ui: &mut Ui, name: &str, value: u32, modifier: i32, fontsize: f32) {
    ui.vertical(|ui| {
        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            ui.add(
                RichTestBuilder::new(&format!("{}({})", name, value))
                    .size(fontsize)
                    .font_family(dnd_font_family())
                    .color(match value {
                        ..=9 => egui::Color32::RED,
                        10 => egui::Color32::WHITE,
                        11.. => egui::Color32::GREEN,
                    })
                    .into_label()
            );
            ui.add(
                RichTestBuilder::new(&format!("{}{}", number_sign(modifier), modifier.abs()))
                    .size(fontsize)
                    .font_family(dnd_font_family())
                    .color(match modifier {
                        ..=-1 => egui::Color32::RED,
                        0 => egui::Color32::WHITE,
                        1.. => egui::Color32::GREEN,
                    })
                    .into_label()
            )
        });
    });
}

fn populate_ability_scores_collapsable(ui: &mut Ui, ability_scores: AbilityScores, character_id: Uuid) {
    let modifiers = ability_scores.ability_score_modifiers();
    ui.collapsing(RichTestBuilder::new("Ability Scores").size(17.0).into_rich_text(), |ui| {
        Grid::new(format!("ability_scores_{}", character_id)).spacing((0.0, 20.0)).min_col_width(80.0).show(ui, |ui| {
            let fontsize = 14.0;
            add_stat_with_layout(ui, "STR", ability_scores.strength, modifiers.strength, fontsize);
            add_stat_with_layout(ui, "DEX", ability_scores.dexterity, modifiers.dexterity, fontsize);
            add_stat_with_layout(ui, "CON", ability_scores.constitution, modifiers.constitution, fontsize);
            ui.end_row();
            add_stat_with_layout(ui, "INT", ability_scores.intelligence, modifiers.intelligence, fontsize);
            add_stat_with_layout(ui, "WIS", ability_scores.wisdom, modifiers.wisdom, fontsize);
            add_stat_with_layout(ui, "CHA", ability_scores.charisma, modifiers.charisma, fontsize);
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
                populate_ability_scores_collapsable(ui, ability_scores, character.id);
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