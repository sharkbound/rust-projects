use eframe::{App, egui, Frame, run_native};
use eframe::egui::{Context, Sense};

// https://github.com/emilk/egui/blob/master/examples
// https://crates.io/crates/eframe
pub struct MainApp;

impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::Window::new("Character Loader").show(ctx, |ui| {
            ui.menu_button("testing", |ui| {});
        });

        egui::TopBottomPanel::top("aba").show(ctx, |ui| {
        });
        // egui::CentralPanel::default().show(ctx, |ui| {
        //
        // });
    }
}

impl MainApp {
    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions::default();
        run_native(Self::name(), options, Box::new(|_cc| Box::new(MainApp)))
    }

    fn name() -> &'static str {
        "Dungeons and Dragons Manager"
    }
}