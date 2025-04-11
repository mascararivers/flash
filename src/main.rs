use iced::widget::{button, column, text};
use iced::{Element, Task};

#[derive(Default)]
struct App {}

#[derive(Clone, Debug)]
enum Message {}

impl App {
    fn view(&self) -> Element<Message> {
        column![].into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }
}

fn main() -> iced::Result {
    iced::application("Flash", App::update, App::view).run()
}
