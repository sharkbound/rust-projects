use iced::{Application, Command, Element, Renderer, Settings, window};
use iced::widget::{container, row, text_input};

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Settings::default()
    })
}

struct App {
    text: String,
}

impl Default for App {
    fn default() -> Self {
        Self { text: String::new() }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Default::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Iced Basic Application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextChanged(text) => {
                self.text = text;
            }
        }


        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        println!();
        let row = row![
            text_input("enter text here", &self.text).on_input(Message::TextChanged)
        ];
        container(row).into()
    }
}