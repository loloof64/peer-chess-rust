mod chessboard;
use chessboard::ChessBoard;

use iced::theme::Theme;
use iced::widget::{button, container, svg, Column};
use iced::Alignment;
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
        let toggle_board_handle = svg::Handle::from_path(format!(
            "{}/resources/images/swap_vertical.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let toggle_board_image = svg(toggle_board_handle)
            .width(Length::Fill)
            .height(Length::Fill);
        container(
            Column::new()
                .align_items(Alignment::Center)
                .spacing(5)
                .push(
                    button(toggle_board_image)
                        .width(40)
                        .height(40)
                        .on_press(Message::ToggleBoardOrientation),
                )
                .push(self.board.clone()),
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
