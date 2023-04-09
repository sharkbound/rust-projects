use std::env;
use eframe::{App, egui, Frame, run_native};
use eframe::egui::{Context};

// https://github.com/emilk/egui/blob/master/examples
// https://crates.io/crates/eframe
pub struct MainApp {
    file_dialog: Option<egui_file::FileDialog>,
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            file_dialog: None
        }
    }

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

    fn check_file_dialog(&mut self, ctx: &Context) {
        if let Some(file_dialog) = &mut self.file_dialog {
            if !file_dialog.show(ctx).selected() {
                return;
            }

            if let Some(file) = file_dialog.path() {
                self.file_dialog = None;
                todo!("add gui elements & proper handling when a json file is selected");
            }
        }
    }
}

impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("aba").show(ctx, |ui| {
            ui.menu_button("Load", |ui| {
                if ui.button("load campaign from file").clicked() {
                    self.set_new_file_dialog();
                    ui.close_menu();
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.check_file_dialog(ctx);
        });
    }
}

impl MainApp {
    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions::default();
        run_native(Self::name(), options, Box::new(|_cc| Box::new(MainApp::new())))
    }

    fn name() -> &'static str {
        "Dungeons and Dragons Manager"
    }
}