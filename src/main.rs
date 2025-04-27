use iced::Length::Fill;
use iced::widget::shader::wgpu::naga::Bytes;
use iced::widget::text::Style;
use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, Font, Task};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Flashcard {
    question: String,
    answers: Vec<String>,
    correct_question: usize, // could be i32 but usize is not negative
    image: Option<String>,   // path
}

#[derive(Clone, Debug, Default)]
enum Menu {
    #[default]
    Main,
    CreateFlashcard,
}

#[derive(Clone, Debug)]
enum Message {
    Cycle,
    Add,
    Remove,
    Back,
    ChangeMenu(Menu),
}

#[derive(Default)]
struct App {
    menu_state: Menu,
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;

impl App {
    fn view(&self) -> Element<Message> {
        let c = match self.menu_state {
            Menu::Main => container(column![
                text("Flash")
                    .size(WINDOW_WIDTH / WINDOW_HEIGHT * 50.0)
                    .font(Font {
                        weight: iced::font::Weight::ExtraBold,
                        ..Default::default()
                    })
                    .style(|_| Style {
                        color: Some(Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0
                        })
                    }),
                row![button(text("create")).on_press(Message::ChangeMenu(Menu::CreateFlashcard))]
            ]),
            Menu::CreateFlashcard => container(button(text("Back")).on_press(Message::Back)),
        };
        container(c).width(Fill).into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeMenu(menu) => {
                self.menu_state = menu;
            }
            Message::Back => {
                self.menu_state = Menu::Main;
            }
            Message::Add => {}
            Message::Remove => {}
            Message::Cycle => {}
        }

        Task::none()
    }
}

fn main() -> iced::Result {
    iced::application("Flash", App::update, App::view)
        .theme(|_| iced::Theme::Dark)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .run()
}
