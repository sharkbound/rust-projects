use eframe::egui;
use eframe::egui::{Align2, Context};
use serde_json::error::Category;
use dndlib::CampaignLoadError;
use crate::ModalAction;


pub(crate) fn create_campaign_load_error_modal(e: CampaignLoadError) -> Option<Box<dyn Fn(&Context) -> ModalAction>> {
    Some(Box::new(move |ctx| {
        let mut modal_end_state = ModalAction::KeepOpen;
        egui::Window::new("Campaign Load Error")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(format!(r#"An error occurred while loading the campaign json file located at: "{:?}""#, e.path()));
                    ui.separator();
                    match &e {
                        CampaignLoadError::FileNotFound(file) => { ui.label("the selected file could not be found"); }
                        CampaignLoadError::ReadFile(file) => { ui.label("An error occurred when trying to read the file"); }
                        CampaignLoadError::FileOpen(file) => { ui.label("An error occurred when trying to open the file"); }
                        CampaignLoadError::Json(file, error) => {
                            match error.classify() {
                                Category::Io => { ui.label(format!("An IO error occurred when trying to read the file")); }
                                Category::Syntax => {
                                    ui.label(format!("A Syntax error occurred at:\n\tLine: {:?}\n\tColumn: {:?}", error.line(), error.column()));
                                }
                                Category::Data => {
                                    ui.label(format!("--- {:?} ---", error));
                                    ui.label(
                                        format!(
                                            "A Data error occurred at:\n\tLine: {:?}\n\tColumn: {:?}\n\n\
                                        It's likely that the json does not conform the campaign save format, lacks some json keys, or was corrupted.",
                                            error.line(), error.column()));
                                }
                                Category::Eof => { ui.label(format!("Reached Unexpected End Of File")); }
                            }
                        }
                    }
                });
                if ui.button("Close").clicked() {
                    modal_end_state = ModalAction::Close;
                }
            });
        modal_end_state
    }))
}
