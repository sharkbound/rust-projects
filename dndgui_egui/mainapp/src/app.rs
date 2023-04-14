use eframe::{App, egui, Frame, run_native};
use eframe::egui::Context;
use dndlib::{DndCampaign};
use crate::enums::ModalAction;
use crate::enums::MainTab;
use crate::{file_dialog_handler, modals};

pub struct MainApp {
    pub(crate) file_dialog: Option<egui_file::FileDialog>,
    pub(crate) campaign: Option<DndCampaign>,
    pub(crate) modal: Option<Box<dyn Fn(&Context) -> ModalAction>>,
    pub(crate) current_maintab: MainTab,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            file_dialog: Default::default(),
            campaign: Default::default(),
            modal: Default::default(),
            current_maintab: Default::default(),
        }
    }
}

impl MainApp {
    fn show_top_menu_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("primary_topbar").show(ctx, |ui| {
            ui.menu_button("Load", |ui| {
                if ui.button("load campaign from file").clicked() {
                    file_dialog_handler::set_new_file_dialog(self);
                    ui.close_menu();
                }
            })
        });
    }
}

impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        match file_dialog_handler::validate_load_file_selection(self, ctx) {
            Ok(_) => {}
            Err(e) => {
                self.modal = modals::create_campaign_load_error_modal(e);
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
                        ui.selectable_value(&mut self.current_maintab, MainTab::Overview, "Overview");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Characters, "Characters");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Notes, "Notes");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Settings, "Settings");
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    match self.current_maintab {
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