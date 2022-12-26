mod chessboard;

use chessboard::ChessBoardBackground;
use iced::theme::Theme;
use iced::widget::{container};
use iced::{executor, Command, Length};
use iced::{Application, Element};

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub struct App {
    background: ChessBoardBackground,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                background: ChessBoardBackground::new(400u16),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced chess experiment")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let bg1 = self.background.view().map(move |_message| Message::None);
        let content = bg1;

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
