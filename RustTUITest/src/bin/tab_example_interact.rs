use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, MouseEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Tabs},
    DefaultTerminal, Frame,
};
use ratatui_interact::traits::ClickRegionRegistry;
use RustTUITest::utils;

struct App {
    selected_tab: usize,
    tab_titles: Vec<&'static str>,
    click_registry: ClickRegionRegistry<usize>,
}

impl App {
    fn new() -> Self {
        Self {
            selected_tab: 0,
            tab_titles: vec!["Home", "Settings", "About", "Quit"],
            click_registry: ClickRegionRegistry::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(std::time::Duration::from_millis(50))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => {
                            self.selected_tab =
                                self.selected_tab.saturating_sub(1);
                        }
                        KeyCode::Right => {
                            self.selected_tab = (self.selected_tab + 1)
                                .min(self.tab_titles.len() - 1);
                        }
                        _ => {}
                    },
                    Event::Mouse(mouse) => {
                        if let MouseEventKind::Down(_) = mouse.kind {
                            // Use registry to check if click hit a tab
                            if let Some(clicked_tab) = self
                                .click_registry
                                .handle_click(mouse.column, mouse.row)
                            {
                                self.selected_tab = *clicked_tab;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
        ]);
        let [tabs_area, content_area] = vertical.areas(frame.area());

        // Clear previous click regions
        self.click_registry.clear();

        // Calculate and register tab positions
        let mut x = tabs_area.x + 1;
        for (i, title) in self.tab_titles.iter().enumerate() {
            let width = title.len() as u16 + 2;
            let rect = Rect::new(x, tabs_area.y + 1, width, 1);
            self.click_registry.register(rect, i);
            x += width + 1;
        }

        // Render tabs
        let tabs = Tabs::new(
            self.tab_titles
                .iter()
                .map(|&t| Line::from(t))
                .collect::<Vec<_>>(),
        )
            .select(self.selected_tab)
            .highlight_style(Style::new().fg(Color::Yellow).bold())
            .block(Block::bordered().title("Clickable Tabs"));

        frame.render_widget(tabs, tabs_area);

        // Content
        let content = format!(
            "Tab: {} ({})\nMouse: enabled | Keys: ←/→/q",
            self.selected_tab, self.tab_titles[self.selected_tab]
        );
        frame.render_widget(
            Block::bordered(),
            content_area,
        );
    }
}

fn main() -> io::Result<()> {
    ratatui::crossterm::execute!(
        io::stdout(),
        ratatui::crossterm::event::EnableMouseCapture
    )?;

    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();

    let _ = ratatui::crossterm::execute!(
        io::stdout(),
        ratatui::crossterm::event::DisableMouseCapture
    );

    result
}