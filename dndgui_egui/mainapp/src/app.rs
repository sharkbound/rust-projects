use std::time::Instant;
use eframe::{App, Frame, run_native};
use eframe::egui::Context;

// https://github.com/emilk/egui/blob/master/examples
// https://crates.io/crates/eframe
pub struct MainApp;

impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        println!("{:?} :: update!", Instant::now())
    }
}


impl MainApp {
    pub fn run() {
        let options = eframe::NativeOptions::default();
        run_native(Self::name(), options, Box::new(|_cc| Box::new(MainApp)));
    }

    fn name() -> &'static str {
        "Dungeons and Dragons Manager"
    }
}