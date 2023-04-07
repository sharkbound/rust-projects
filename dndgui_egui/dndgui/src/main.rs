use dndlib::{AttributeStats, Character, DndCampaign};
use mainapp;

// surrealdb: https://www.youtube.com/watch?v=iOyvum0D3LM&list=PLvuQflRR4UzYhl-CDmmuqzLwuywWi7vo2

fn main() {
    println!("Starting Dungeons and Dragons GUI...");
    let char = Character::new("james", AttributeStats::new(10, 18, 8, 5, 4, 7));
    let mut campaign = DndCampaign::default();
    campaign.add_character(char);
    println!("{:#?}", campaign);
    mainapp::MainApp::run().unwrap();
}