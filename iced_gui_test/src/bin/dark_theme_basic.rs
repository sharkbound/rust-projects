use iced::{Application, Command, Element, Renderer, Settings, theme, Theme, window, Alignment, Length};
use iced::widget::{button, container, row, text_input, column, radio, vertical_rule, horizontal_rule, horizontal_space, vertical_space};

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
    selected_radio: RadioOption,
    current_theme: CurrentTheme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            text: String::new(),
            selected_radio: Default::default(),
            current_theme: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    ButtonClicked,
    TextInputSubmitted,
    RadioValueChanged(RadioOption),
    ThemeChanged(CurrentTheme),
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
enum RadioOption {
    #[default]
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
enum CurrentTheme {
    Light,
    #[default]
    Dark,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
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
            Message::ButtonClicked => {}
            Message::TextInputSubmitted => {
                println!("{}", self.text);
            }
            Message::RadioValueChanged(value) => {
                self.selected_radio = value;
            }
            Message::ThemeChanged(new_theme) => {
                self.current_theme = new_theme;
            }
        }


        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let textinput = text_input("enter text here", &self.text)
            .on_submit(Message::TextInputSubmitted)
            .on_input(Message::TextChanged)
            .width(Length::Fixed(400.0));

        let button1 = button("Click Me").on_press(Message::ButtonClicked);

        let radio1 = radio("Radio 1", RadioOption::One, Some(self.selected_radio), Message::RadioValueChanged);
        let radio2 = radio("Radio 2", RadioOption::Two, Some(self.selected_radio), Message::RadioValueChanged);
        let radio3 = radio("Radio 3", RadioOption::Three, Some(self.selected_radio), Message::RadioValueChanged);

        let theme_light_radio = radio("Light Theme", CurrentTheme::Light, Some(self.current_theme), Message::ThemeChanged);
        let theme_dark_radio = radio("Dark Theme", CurrentTheme::Dark, Some(self.current_theme), Message::ThemeChanged);

        let row1 = row![textinput];
        let row2 = row![button1];
        let row_radios = row![radio1, radio2, radio3].spacing(10);
        let row_theme_radios = row![theme_light_radio, theme_dark_radio].spacing(10);

        let left_right_split = row![
            button("Left"),
            horizontal_space(Length::Fill),
            button("Right"),
        ];

        let content = container(column![
            row1,
            vertical_space(10),
            row2,
            vertical_space(10),
            row_radios,
            vertical_space(10),
            row_theme_radios,
            vertical_space(10),
            left_right_split,
        ].align_items(Alignment::Center));

        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            // .style(iced::theme::Container::Box)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        match self.current_theme {
            CurrentTheme::Light => Theme::Light,
            CurrentTheme::Dark => Theme::Dark,
        }
    }
}

