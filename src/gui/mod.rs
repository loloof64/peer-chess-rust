mod chessboard;
use chessboard::ChessBoard;

use iced::theme::Theme;
use iced::widget::{container, column, button};
use iced::Element;
use iced::{Length, Sandbox};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleBoardOrientation,
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
        String::from("Peer chess")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleBoardOrientation => self.board.toggle_orientation(),
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                button("Turn board").on_press(Message::ToggleBoardOrientation),
                self.board.clone(),
            ]
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
