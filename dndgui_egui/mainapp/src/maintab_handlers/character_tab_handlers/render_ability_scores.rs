use eframe::egui;
use eframe::egui::{Grid, Layout, Ui};
use uuid::Uuid;
use dndlib::AbilityScores;
use crate::helpers::{dnd_font_family, RichTestBuilder};
use crate::maintab_handlers::utils::number_sign;


pub(crate) fn render_ability_score_stat(ui: &mut Ui, name: &str, value: u32, modifier: i32, fontsize: f32) {
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


pub(crate) fn render_ability_scores_collapsable(ui: &mut Ui, ability_scores: &AbilityScores, character_id: Uuid) {
    let modifiers = ability_scores.ability_score_modifiers();
    ui.collapsing(RichTestBuilder::new("Ability Scores").size(17.0).into_rich_text(), |ui| {
        Grid::new(format!("ability_scores_{}", character_id)).spacing((0.0, 20.0)).min_col_width(80.0).show(ui, |ui| {
            let fontsize = 14.0;
            render_ability_score_stat(ui, "STR", ability_scores.strength, modifiers.strength, fontsize);
            render_ability_score_stat(ui, "DEX", ability_scores.dexterity, modifiers.dexterity, fontsize);
            render_ability_score_stat(ui, "CON", ability_scores.constitution, modifiers.constitution, fontsize);
            ui.end_row();
            render_ability_score_stat(ui, "INT", ability_scores.intelligence, modifiers.intelligence, fontsize);
            render_ability_score_stat(ui, "WIS", ability_scores.wisdom, modifiers.wisdom, fontsize);
            render_ability_score_stat(ui, "CHA", ability_scores.charisma, modifiers.charisma, fontsize);
        });
    });
}