mod chessboard;
use chessboard::ChessBoard;

use iced::theme::Theme;
use iced::widget::container;
use iced::Element;
use iced::{Length, Sandbox};

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub struct App {
    board: ChessBoard,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            board: ChessBoard::new(400u16),
        }
    }

    fn title(&self) -> String {
        String::from("Iced chess experiment")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::None => println!("Button clicked !"),
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            self.board,
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}
