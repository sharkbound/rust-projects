use std::collections::HashMap;
use eframe::{App, egui, Frame, run_native};
use eframe::egui::{Context};
use dndlib::{DndCampaign};
use crate::enums::ModalAction;
use crate::enums::MainTab;
use crate::{file_dialog_handler, modals, topmenu};
use crate::maintab_handlers::characters_tab_handler::render_maintab_characters;
use crate::maintab_handlers::overview_tab_handler::render_maintab_overview;

pub struct MainApp {
    pub(crate) file_dialog: Option<egui_file::FileDialog>,
    pub(crate) campaign: Option<DndCampaign>,
    pub(crate) modal: Option<Box<dyn Fn(&Context) -> ModalAction>>,
    pub(crate) current_maintab: MainTab,
    pub(crate) checked_store: HashMap<String, bool>,
    initfonts: bool,
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

impl Default for MainApp {
    fn default() -> Self {
        Self {
            file_dialog: Default::default(),
            campaign: Default::default(),
            modal: Default::default(),
            current_maintab: Default::default(),
            initfonts: true,
            checked_store: HashMap::new(),
        }
    }
}


impl App for MainApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.initfonts {
            crate::helpers::setup_fonts(ctx);
            self.initfonts = false;
        }

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

        topmenu::show_top_menu(ctx, self);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                egui::TopBottomPanel::top("tab_topbar").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.current_maintab, MainTab::Overview, "Overview");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Characters, "Characters");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Locations, "Locations");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Notes, "Notes");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Settings, "Settings");
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    match self.current_maintab {
                        MainTab::Overview => { render_maintab_overview(ctx, ui, self); }
                        MainTab::Characters => { render_maintab_characters(ctx, ui, self); }
                        MainTab::Notes => { ui.label("Notes"); }
                        MainTab::Settings => { ui.label("Settings"); }
                        MainTab::Locations => { ui.label("Locations"); }
                    }
                });
            });
        });
    }
}
