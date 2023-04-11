use std::env;
use eframe::{App, egui, Frame, run_native};
use eframe::egui::{Context};
use eframe::emath::Align2;
use serde_json::error::Category;
use dndlib::{CampaignLoadError, DndCampaign, load_campaign};
use crate::MainTab;

// https://github.com/emilk/egui/blob/master/examples
// https://crates.io/crates/eframe
enum ModalAction {
    Close,
    KeepOpen,
}

#[derive(Default)]
pub struct MainApp {
    file_dialog: Option<egui_file::FileDialog>,
    campaign: Option<DndCampaign>,
    modal: Option<Box<dyn Fn(&Context) -> ModalAction>>,
    maintab: MainTab,
}

impl MainApp {
    // pub fn new() -> Self {
    //     Self {
    //         file_dialog: None,
    //         campaign: None,
    //         modal: None,
    //     }
    // }

    fn set_new_file_dialog(&mut self) {
        let mut file_dialog = egui_file::FileDialog::open_file(Some(env::current_dir().unwrap()))
            .filter(Box::new(|path| match path.extension() {
                Some(extension) => {
                    extension.to_string_lossy().eq_ignore_ascii_case("json")
                }
                None => false,
            }));

        file_dialog.open();
        self.file_dialog = Some(file_dialog);
    }

    fn validate_load_file_selection(&mut self, ctx: &Context) -> Result<(), CampaignLoadError> {
        match &mut self.file_dialog {
            None => Ok(()),
            Some(file_dialog) => {
                if !file_dialog.show(ctx).selected() {
                    return Ok(());
                }
                match file_dialog.path() {
                    None => Ok(()),
                    Some(file) => {
                        self.file_dialog = None;
                        match load_campaign(file.as_path()) {
                            Ok(campaign) => {
                                self.campaign = Some(campaign);
                                Ok(())
                            }
                            Err(e) => {
                                Err(e)
                            }
                        }
                    }
                }
            }
        }
    }

    fn show_top_menu_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("primary_topbar").show(ctx, |ui| {
            ui.menu_button("Load", |ui| {
                if ui.button("load campaign from file").clicked() {
                    self.set_new_file_dialog();
                    ui.close_menu();
                }
            })
        });
    }

    fn create_campaign_load_error_modal(e: CampaignLoadError) -> Option<Box<dyn Fn(&Context) -> ModalAction>> {
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
}

impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        match self.validate_load_file_selection(ctx) {
            Ok(_) => {}
            Err(e) => {
                self.modal = Self::create_campaign_load_error_modal(e);
            }
        }

        match &self.modal {
            None => {}
            Some(handler) => {
                match handler(ctx) {
                    ModalAction::KeepOpen => return,
                    ModalAction::Close => {
                        self.modal = None;
                        return;
                    }
                }
            }
        }

        self.show_top_menu_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                egui::TopBottomPanel::top("tab_topbar").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.maintab, MainTab::Overview, "Overview");
                        ui.selectable_value(&mut self.maintab, MainTab::Characters, "Characters");
                        ui.selectable_value(&mut self.maintab, MainTab::Notes, "Notes");
                        ui.selectable_value(&mut self.maintab, MainTab::Settings, "Settings");
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    match self.maintab {
                        MainTab::Overview => { ui.label("Overview"); }
                        MainTab::Characters => { ui.label("Characters"); }
                        MainTab::Notes => { ui.label("Notes"); }
                        MainTab::Settings => { ui.label("Settings"); }
                    }
                });
            });
        });
    }
}

impl MainApp {
    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions::default();
        run_native(Self::name(), options, Box::new(|_cc| Box::new(MainApp::default())))
    }

    fn name() -> &'static str {
        "Dungeons and Dragons Manager"
    }
}