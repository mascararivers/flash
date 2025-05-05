use directories::ProjectDirs;
use iced::Length::Fill;
use iced::border::Radius;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::text::Style as TextStyle;
use iced::widget::{Container, button, column, container, row, text, text_input};
use iced::{Border, Color, Element, Font, Task};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone)]
struct Flashcard {
    question: String,
    answers: [String; 4],
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
    Add(String, [String; 4], usize, Option<String>),
    Remove,
    Back,
    ChangeMenu(Menu),
    ChangeTitle(String),
    ChangeContent(String),
    ChangeImageString(Option<String>),
}

struct App {
    menu_state: Menu,
    flashcards: Vec<Flashcard>,
    project_dir: ProjectDirs,
    config: PathBuf,
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;

impl Default for App {
    fn default() -> Self {
        let dirs = ProjectDirs::from("", "", "flash").unwrap();

        let config_path = dirs.config_dir();
        let config_file = config_path.join("flashcards.json");

        if !config_path.exists() {
            std::fs::create_dir_all(config_path).unwrap();

            let mut file = std::fs::File::create(&config_file).unwrap();
            file.write_all(b"{}").unwrap();
        }

        let read = fs::read_to_string(&config_file).unwrap();

        let flashcard_struct: Vec<Flashcard> = serde_json::from_str(read.as_str()).unwrap();

        App {
            project_dir: dirs,
            menu_state: Menu::Main,
            flashcards: flashcard_struct,
            config: config_file,
        }
    }
}

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

        let a_row = row![];

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
            Menu::CreateFlashcard => container(row![
                // text_input().on_input(Message::)
                button(text("Add Answer")).on_press(),
            ]),
            //button(text("Back")).on_press(Message::Back)
        };
        container(c).width(Fill).center(Fill).into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeMenu(menu) => {
                self.menu_state = menu;
            }
            Message::Back => {
                self.menu_state = Menu::Main;
            }
            Message::Add(question, answers, correct_question, image) => {
                let file_contents = std::fs::read_to_string(&self.config).unwrap();

                let data: Vec<Flashcard> = serde_json::from_str(&file_contents).unwrap();

                self.flashcards = data;

                let flashcard = Flashcard {
                    question: question,
                    answers: answers,
                    correct_question: correct_question,
                    image: image,
                };

                self.flashcards.push(flashcard.clone());

                let updated_cards = serde_json::to_string_pretty(&self.flashcards).unwrap();

                let _ = std::fs::write(&self.config, updated_cards.as_bytes());
            }
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
