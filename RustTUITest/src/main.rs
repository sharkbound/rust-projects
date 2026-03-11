pub mod utils;

use crate::KeyAction::Quit;
use crate::utils::create_click_region_registry;
use crossterm::event::{Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
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
                String::from("Characters (F1)"),
                String::from("Items (F2)"),
                String::from("Combat (F3)"),
                String::from("Logs (F4)"),
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
            match event {
                Event::Key(key) => {
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
                Event::Mouse(e) if matches!(e.kind, MouseEventKind::Down(MouseButton::Left)) => {
                    self.handle_mouse_click(e);
                }
                _ => {}
            }
        }
    }

    fn handle_mouse_click(&mut self, event: MouseEvent) {
        if let Some(tab_index) = self.click_interaction_registry.handle_click(event.column, event.row) {
            self.current_tab_index = *tab_index;
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
        let [tabs_area, content_area] = vertical.areas(frame.area());
        let tabs = Tabs::new(
            self.tab_titles
                .iter()
                .map(|x| Line::from(x.as_ref()))
                .collect::<Vec<_>>(),
        )
        .select(self.current_tab_index)
        .highlight_style(Style::new().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));

        // Update click regions
        self.click_interaction_registry.clear();
        let mut x = tabs_area.x + 1; // offset for border
        for (i, title) in self.tab_titles.iter().enumerate() {
            let width = title.len() as u16 + 2;
            let rect = Rect::new(x, tabs_area.y + 1, width, 1);
            self.click_interaction_registry.register(rect, i);
            x += width + 1;
        }

        frame.render_widget(tabs, tabs_area);

        frame.render_widget(
            Paragraph::new("Hello, world!")
                .centered()
                .block(Block::bordered()),
            content_area,
        );
    }
}

enum KeyAction {
    Quit,
    None,
}
