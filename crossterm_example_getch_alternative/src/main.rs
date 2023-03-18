use crossterm::{cursor, execute};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

fn main() {
    let mut stdout = std::io::stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, Clear(ClearType::All)).unwrap();
    loop {
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        match read().unwrap() {
            Event::Key(e) => {
                if let KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. } = e {
                    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(5, 5), Print("BYE!")).unwrap();
                    break;
                }
                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(5, 5), Print(format!("GOT KEY: {:?}, WITH MODS: {:?}", e.code, e.modifiers))).unwrap();
            }
            _ => {}
        }
    }
    disable_raw_mode().unwrap();
}