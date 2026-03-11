pub mod utils;

use crate::KeyAction::Quit;
use crate::utils::create_click_region_registry;
use crossterm::event::{Event, KeyCode};
use crossterm::{execute, terminal::enable_raw_mode};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};
use ratatui_interact::prelude::*;
use std::io;

fn main() -> color_eyre::Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Enable mouse capture
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    // Your app logic here...
    color_eyre::install()?;

    TuiApp::new().start()?;

    // Cleanup: Disable mouse capture before exiting
    execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;

    Ok(())
}

struct TuiApp {
    tab_titles: Vec<String>,
    current_tab_index: usize,
    click_interaction_registry: ClickRegionRegistry<usize>,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            tab_titles: vec![
                String::from("Characters"),
                String::from("Items"),
                String::from("Combat"),
                String::from("Logs"),
            ],
            current_tab_index: 0,
            click_interaction_registry: create_click_region_registry(),
        }
    }

    fn start(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        ratatui::run(|t| self.run(t))?;
        ratatui::restore();
        Ok(())
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|t| self.render(t))?;
            let event = crossterm::event::read()?;
            if let Event::Key(key) = event {
                if key.is_press() {
                    match self.handle_key_press(key) {
                        KeyAction::Quit => return Ok(()),
                        KeyAction::None => {}
                    }
                } else {
                    match self.handle_key_release(key) {
                        KeyAction::Quit => return Ok(()),
                        KeyAction::None => {}
                    }
                }
            }
            // dbg!(&event);
        }
    }

    fn handle_key_press(&mut self, key: crossterm::event::KeyEvent) -> KeyAction {
        match key.code {
            KeyCode::Char('q') => return KeyAction::Quit,
            KeyCode::F(1) => self.current_tab_index = 0,
            KeyCode::F(2) => self.current_tab_index = 1,
            KeyCode::F(3) => self.current_tab_index = 2,
            KeyCode::F(4) => self.current_tab_index = 3,
            _ => {}
        };
        KeyAction::None
    }

    fn handle_key_release(&mut self, key: crossterm::event::KeyEvent) -> KeyAction {
        KeyAction::None
    }

    fn render(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
        let [tabs_rect, content_rect] = vertical.areas(frame.area());
        let tabs = Tabs::new(
            self.tab_titles
                .iter()
                .map(|x| Line::from(x.as_ref()))
                .collect::<Vec<_>>(),
        )
        .select(self.current_tab_index)
        .highlight_style(Style::new().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));

        frame.render_widget(tabs, tabs_rect);

        frame.render_widget(
            Paragraph::new("Hello, world!").centered()
                .block(Block::bordered()),
            content_rect,
        );
    }
}

enum KeyAction {
    Quit,
    None,
}
