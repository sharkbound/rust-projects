// pub mod utils;

use crossterm::execute;
use crossterm::terminal::enable_raw_mode;
use ratatui_interact::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::{DefaultTerminal, Frame};

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

fn render(frame: &mut Frame) {
    // click region registry
    // let mut registry = create_click_region_registry();




    let block = Block::bordered().title("Ratatui").borders(Borders::ALL);
    let greeting = Paragraph::new("Hello from Ratatui~")
        .centered()
        .yellow()
        .block(block.clone());
    let tabs = Tabs::new(vec!["Tab 1", "Tab 2", "Tab 3"]).block(block.clone());
    frame.render_widget(tabs, frame.area());
    frame.render_widget(greeting, frame.area());
}
