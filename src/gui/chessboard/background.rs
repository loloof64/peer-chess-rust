use iced::widget::canvas::{self, Cursor, Event, Geometry};
use iced::widget::canvas::{Cache, Canvas, Path, Text};
use iced::{event, mouse, Color, Element, Length, Point, Rectangle, Size, Theme, Font};

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Default)]
pub enum Interaction {
    #[default]
    None,
}

pub struct ChessBoardBackground {
    size: u16,
    white_cell_color: Color,
    black_cell_color: Color,
    cells_cache: Cache,
    text_cache: Cache,
}

impl ChessBoardBackground {
    pub fn new(size: u16) -> Self {
        Self {
            size,
            white_cell_color: Color::from_rgb8(0xFF, 0xDE, 0xAD),
            black_cell_color: Color::from_rgb8(0xCD, 0x85, 0x3F),
            cells_cache: Cache::default(),
            text_cache: Cache::default(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(self.size))
            .height(Length::Units(self.size))
            .into()
    }
}

impl canvas::Program<Message> for ChessBoardBackground {
    type State = Interaction;

    fn update(
        &self,
        _interaction: &mut Interaction,
        _event: Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _interaction: &Interaction,
        _theme: &Theme,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let size = self.size as f32;
        let cells = self.cells_cache.draw(
            Size {
                width: size,
                height: size,
            },
            |frame| {
                let background = Path::rectangle(
                    Point::new(0f32, 0f32),
                    frame.size(),
                );
                frame.fill(&background, Color::from_rgb8(0x00, 0x80, 0xFF));

                (0..8).for_each(|row| {
                    (0..8).for_each(|col| {
                        let cells_size = (self.size as f32) * 0.111;
                        let is_white_cell = (row + col) % 2 == 0;
                        let cell_color = if is_white_cell {
                            self.white_cell_color
                        } else {
                            self.black_cell_color
                        };
                        let x = cells_size * (col as f32 + 0.5);
                        let y = cells_size * (row as f32 + 0.5);

                        let bounds =
                            Path::rectangle(Point::new(x, y), Size::new(cells_size, cells_size));
                        frame.fill(&bounds, cell_color);
                    });
                });
            },
        );
        let texts = self.text_cache.draw(
            Size {
                width: size,
                height: size,
            },
            |frame| {
                let files = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
                let ranks = vec!["8", "7", "6", "5", "4", "3", "2", "1"];

                let color = Color::from_rgb8(0xFF, 0xFF, 0x00);

                files.into_iter().enumerate().for_each(|(col, file)| {
                    let cells_size = (self.size as f32) * 0.111;
                    let font_size = cells_size * 0.4;
                    let x = cells_size * (col as f32 + 1.0);
                    let y1 = cells_size * 0.25;
                    let y2 = cells_size * 8.75;

                    let text1 = Text {
                        content: file.into(),
                        position: Point::new(x, y1),
                        color,
                        size: font_size,
                        font: Font::default(),
                        horizontal_alignment: iced::alignment::Horizontal::Center,
                        vertical_alignment: iced::alignment::Vertical::Center,
                    };
                    frame.fill_text(text1);

                    let text2 = Text {
                        content: file.into(),
                        position: Point::new(x, y2),
                        color,
                        size: font_size,
                        font: Font::default(),
                        horizontal_alignment: iced::alignment::Horizontal::Center,
                        vertical_alignment: iced::alignment::Vertical::Center,
                    };
                    frame.fill_text(text2);
                });

                ranks.into_iter().enumerate().for_each(|(row, rank)| {
                    let cells_size = (self.size as f32) * 0.111;
                    let font_size = cells_size * 0.4;
                    let x1 = cells_size * 0.25;
                    let x2 = cells_size * 8.75;
                    let y = cells_size * (row as f32 + 1.0);

                    let text1 = Text {
                        content: rank.into(),
                        position: Point::new(x1, y),
                        color,
                        size: font_size,
                        font: Font::default(),
                        horizontal_alignment: iced::alignment::Horizontal::Center,
                        vertical_alignment: iced::alignment::Vertical::Center,
                    };
                    frame.fill_text(text1);

                    let text2 = Text {
                        content: rank.into(),
                        position: Point::new(x2, y),
                        color,
                        size: font_size,
                        font: Font::default(),
                        horizontal_alignment: iced::alignment::Horizontal::Center,
                        vertical_alignment: iced::alignment::Vertical::Center,
                    };
                    frame.fill_text(text2);
                });
            },
        );
        vec![cells, texts]
    }

    fn mouse_interaction(
        &self,
        _interaction: &Interaction,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}
