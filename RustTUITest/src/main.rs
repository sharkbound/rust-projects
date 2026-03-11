pub mod utils;

use std::io;
use crossterm::execute;
use crossterm::terminal::enable_raw_mode;
use ratatui_interact::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::{DefaultTerminal, Frame};
use crate::utils::create_click_region_registry;

fn main() -> color_eyre::Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Enable mouse capture
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    // Your app logic here...
    color_eyre::install()?;

    ratatui::run(app)?;

    // Cleanup: Disable mouse capture before exiting
    execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;

    Ok(())
}

struct App {
    tab_titles: Vec<String>,
    current_tab_index: usize,
    click_interaction_registry: ClickRegionRegistry<usize>,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab_titles: vec![String::from("Characters"), String::from("Items"), String::from("Combat"), String::from("Logs")],
            current_tab_index: 0,
            click_interaction_registry: create_click_region_registry(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|t| self.render(t))?;
            // todo
        }
    }

    fn render(&mut self, frame: &mut Frame) {

    }
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if false
        /* crossterm::event::read()?.is_key_press()*/
        {
            break Ok(());
        }
    }
}
