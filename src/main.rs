use directories::ProjectDirs;
use iced::{
    Border, Color, Element, Font,
    Length::{self, Fill},
    Task,
    border::Radius,
    widget::{
        button, button::Style as ButtonStyle, column, container, radio, row, text,
        text::Style as TextStyle, text_input,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Serialize, Deserialize, Default, Clone)]
struct Flashcard {
    question: String,
    answers: [String; 4],
    correct_question: usize,
    image: Option<String>,
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
    ChangeNewAnswer(String),
    AddAnswer,
    ChangeQuestion(String),
    ChangeAnswer(usize, String),
    RemoveAnswer(usize),
    SelectCorrect(usize),
    SubmitFlashcard,
}

struct App {
    menu_state: Menu,
    flashcards: Vec<Flashcard>,
    project_dir: ProjectDirs,
    new_question: String,
    new_answer: String,
    new_answers: Vec<String>,
    new_correct_index: Option<usize>,
    new_image: Option<String>,
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 400.0;

macro_rules! full_centered {
    ($child:expr) => {
        container($child)
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
    };
}

impl Default for App {
    fn default() -> Self {
        let dirs = ProjectDirs::from("", "", "flash").unwrap();

        let config_path = dirs.config_dir();
        let config_file = config_path.join("flashcards.json");

        if !config_file.exists() {
            std::fs::create_dir_all(config_path).unwrap();

            let mut file = File::create(&config_file).expect("writing failed");

            // properly format the default flashcard JSON
            let default = vec![Flashcard::default()];
            let default_contents = serde_json::to_string_pretty(&default).unwrap();

            file.write_all(default_contents.as_bytes()).unwrap();
        }

        let read = fs::read_to_string(&config_file).unwrap();
        let flashcard_struct: Vec<Flashcard> = serde_json::from_str(read.as_str()).unwrap();

        App {
            project_dir: dirs,
            menu_state: Menu::Main,
            flashcards: flashcard_struct,
            new_question: String::new(),
            new_answer: String::new(),
            new_answers: Vec::new(),
            new_correct_index: None,
            new_image: None,
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
            Menu::CreateFlashcard => {
                let mut answer_section = column![].spacing(10);

                for (idx, answer) in self.new_answers.iter().enumerate() {
                    let remove_button = if self.new_answers.len() > 2 {
                        button("Remove flashcard").on_press(Message::RemoveAnswer(idx))
                    } else {
                        button("Remove flashcard").padding(5)
                    };

                    answer_section = answer_section.push(row![
                        text_input(&format!("Answer {}", idx + 1), answer)
                            .on_input(move |s| Message::ChangeAnswer(idx, s))
                            .padding(5)
                            .width(Fill),
                        remove_button,
                        radio("Correct", idx, self.new_correct_index, move |_| {
                            Message::SelectCorrect(idx)
                        })
                    ]);
                }

                if self.new_answers.len() < 4 {
                    answer_section = answer_section.push(row![
                        text_input("Add answer…", &self.new_answer)
                            .on_input(Message::ChangeNewAnswer)
                            .on_submit(Message::AddAnswer)
                            .padding(5)
                            .width(Fill),
                        button("+")
                            .on_press_maybe(
                                (!self.new_answer.trim().is_empty() && self.new_answers.len() < 4)
                                    .then(|| Message::AddAnswer)
                            )
                            .padding(5),
                    ]);
                }

                let form_complete = !self.new_question.trim().is_empty()
                    && self.new_answers.len() >= 2
                    && self.new_correct_index.is_some();

                let save_button = button("Save Flashcard")
                    .on_press_maybe(form_complete.then(|| Message::SubmitFlashcard))
                    .padding(10)
                    .width(Length::Fixed(200.0));

                let content = column![
                    text_input("Question…", &self.new_question)
                        .on_input(Message::ChangeQuestion)
                        .padding(5)
                        .width(Fill),
                    answer_section,
                    save_button,
                ]
                .spacing(20)
                .padding(20)
                .max_width(600);

                container(content)
            }
        };
        full_centered!(c).into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeMenu(menu) => {
                self.menu_state = menu;
            }
            Message::Back => {
                self.menu_state = Menu::Main;

                // Reset creation state
                self.new_question.clear();
                self.new_answer.clear();
                self.new_answers.clear();
                self.new_correct_index = None;
                self.new_image = None;
            }
            Message::Add(question, answers, correct_question, image) => {
                let config_path = self.project_dir.config_dir().join("flashcards.json");
                let file_contents = fs::read_to_string(&config_path).unwrap();

                let data: Vec<Flashcard> = serde_json::from_str(&file_contents).unwrap();

                self.flashcards = data;

                let flashcard = Flashcard {
                    question,
                    answers,
                    correct_question,
                    image,
                };

                self.flashcards.push(flashcard.clone());

                let updated_cards = serde_json::to_string_pretty(&self.flashcards).unwrap();

                fs::write(config_path, updated_cards.as_bytes()).unwrap();
            }
            Message::ChangeTitle(s) => {
                self.new_question = s;
            }
            Message::ChangeContent(s) => {
                self.new_answer = s;
            }
            Message::ChangeImageString(s) => {
                self.new_image = s;
            }
            Message::AddAnswer => {
                if self.new_answers.len() < 4 && !self.new_answer.is_empty() {
                    self.new_answers.push(self.new_answer.clone());
                    self.new_answer.clear();
                }
            }
            Message::SelectCorrect(index) => {
                if index < self.new_answers.len() {
                    self.new_correct_index = Some(index);
                }
            }
            Message::Cycle => {}
            Message::Remove => {}
            _ => todo!(),
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
