use std::io::stdout;
use crossterm::{
    event::{self, Event, MouseEventKind, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};

fn main() -> std::io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            let text = "Click anywhere with your mouse!";
            f.render_widget(Paragraph::new(text), f.area());
        })?;

        if let Event::Mouse(mouse) = event::read()? {
            if let MouseEventKind::Down(_) = mouse.kind {
                break; // Exit on any mouse click
            }
        }
    }

    // Cleanup
    execute!(terminal.backend_mut(), DisableMouseCapture, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}