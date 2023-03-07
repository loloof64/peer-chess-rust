mod chessboard;
use chessboard::ChessBoard;

use iced::theme::Theme;
use iced::widget::{button, container, svg, Column};
use iced::Element;
use iced::Length;
use iced::{Alignment, Application, Command};
use pleco::Board;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleBoardOrientation,
    UpdateBoardPosition(String),
}

pub struct App {
    game: Board,
    black_at_bottom: bool,
}

impl Application for App {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                game: Board::default(),
                black_at_bottom: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Peer chess")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleBoardOrientation => {
                self.black_at_bottom = ! self.black_at_bottom;
                Command::none()
            },
            Message::UpdateBoardPosition(new_fen) => {
                if let Ok(new_logic) =  Board::from_fen(&new_fen) {
                    self.game = new_logic;
                }
                Command::none()
            }
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
        let mut board = ChessBoard::new(400u16);
        board.set_game(self.game.clone());
        board.set_orientation(self.black_at_bottom);
        board.set_on_new_position(Box::new(|new_fen| Message::UpdateBoardPosition(new_fen)));
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
                .push(board),
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
