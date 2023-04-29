use std::borrow::Cow;
use eframe::{App, egui, Frame, run_native};
use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, SelectableLabel};
use dndlib::{DndCampaign};
use crate::enums::ModalAction;
use crate::enums::MainTab;
use crate::{file_dialog_handler, modals, topmenu};
use crate::maintab_handlers::overview_tab_handler::render_maintab_overview;

pub struct MainApp {
    pub(crate) file_dialog: Option<egui_file::FileDialog>,
    pub(crate) campaign: Option<DndCampaign>,
    pub(crate) modal: Option<Box<dyn Fn(&Context) -> ModalAction>>,
    pub(crate) current_maintab: MainTab,
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

    fn setup_fonts(ctx: &Context) {
        let mut fontdefs = FontDefinitions::default();
        // fontdefs.font_data.insert("dnd".into(), FontData::from_static(include_bytes!("../../Dalelands.ttf")));
        fontdefs.families.insert(FontFamily::Name("dnd".into()), vec!["Hack".into()]);
        ctx.set_fonts(fontdefs);
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
        }
    }
}


impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.initfonts {
            Self::setup_fonts(ctx);
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
                        ui.selectable_value(&mut self.current_maintab, MainTab::Notes, "Notes");
                        ui.selectable_value(&mut self.current_maintab, MainTab::Settings, "Settings");
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    match self.current_maintab {
                        MainTab::Overview => { render_maintab_overview(ctx, ui, self); }
                        MainTab::Characters => { ui.label("Characters"); }
                        MainTab::Notes => { ui.label("Notes"); }
                        MainTab::Settings => { ui.label("Settings"); }
                    }
                });
            });
        });
    }
}