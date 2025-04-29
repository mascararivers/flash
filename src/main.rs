use std::fs::{create_dir, create_dir_all};
use std::io::Write;
use std::path::Path;

use directories::ProjectDirs;
use iced::Length::Fill;
use iced::border::Radius;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::text::Style as TextStyle;
use iced::widget::{button, column, container, row, text};
use iced::{Border, Color, Element, Font, Task};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
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
    flashcards: Vec<Flashcard>,
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;

impl App {
    fn view(&self) -> Element<Message> {
        let button_style = ButtonStyle {
            text_color: Color::WHITE,
            border: Border {
                color: Color::WHITE,
                width: 5.0,
                radius: Radius {
                    top_left: 5.0,
                    top_right: 5.0,
                    bottom_left: 5.0,
                    bottom_right: 5.0,
                },
            },
            background: None,
            ..Default::default()
        };

        let c = match self.menu_state {
            Menu::Main => container(column![
                text("Flash")
                    .size(WINDOW_WIDTH / WINDOW_HEIGHT * 50.0)
                    .font(Font {
                        weight: iced::font::Weight::ExtraBold,
                        ..Default::default()
                    })
                    .style(|_| TextStyle {
                        color: Some(Color::WHITE)
                    }),
                row![
                    button(text("create"))
                        .on_press(Message::ChangeMenu(Menu::CreateFlashcard))
                        .style(move |_, _| button_style)
                        .width(150.0)
                        .padding(25)
                ]
            ]),
            Menu::CreateFlashcard => container(button(text("Back")).on_press(Message::Back)),
        };
        container(c).width(Fill).center(Fill).into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let flashcards_path = ProjectDirs::from("", "", "flash")
            .unwrap()
            .config_dir()
            .join("flashcards.json");

        match message {
            Message::ChangeMenu(menu) => {
                self.menu_state = menu;
            }
            Message::Back => {
                self.menu_state = Menu::Main;
            }
            Message::Add => {
                let file_contents = std::fs::read_to_string(&flashcards_path).unwrap();

                let data: Vec<Flashcard> = serde_json::from_str(&file_contents).unwrap();

                self.flashcards = data;

                let flashcard = Flashcard {
                    ..Default::default()
                };

                self.flashcards.push(flashcard.clone());

                let updated_cards = serde_json::to_string_pretty(&self.flashcards).unwrap();

                let _ = std::fs::write(flashcards_path, updated_cards.as_bytes());
            }
            Message::Remove => {}
            Message::Cycle => {}
        }

        Task::none()
    }
}

fn main() -> iced::Result {
    let dirs = ProjectDirs::from("", "", "flash").unwrap();
    let config_path = dirs.config_dir();

    if !config_path.exists() {
        std::fs::create_dir_all(config_path).unwrap();
        let config_file = config_path.join("flashcards.json");

        let mut file = std::fs::File::create(config_file).unwrap();
        file.write_all(b"{}").unwrap();
    }

    iced::application("Flash", App::update, App::view)
        .theme(|_| iced::Theme::Dark)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .run()
}
