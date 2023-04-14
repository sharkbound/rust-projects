use mainapp;

// surrealdb: https://www.youtube.com/watch?v=iOyvum0D3LM&list=PLvuQflRR4UzYhl-CDmmuqzLwuywWi7vo2

// https://github.com/emilk/egui/blob/master/examples
// https://crates.io/crates/eframe

fn main() {
    println!("Starting Dungeons and Dragons GUI...");
    mainapp::MainApp::run().unwrap();
}