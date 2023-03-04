use iced::{Color, Element, Length, Point, Rectangle, Size};

use iced_native::renderer::BorderRadius;
use iced_native::{layout, renderer, Widget};

#[derive(Clone, Copy)]
pub struct ChessBoard {
    size: u16,
    background_color: Color,
    white_cell_color: Color,
    black_cell_color: Color,
    //white_turn: bool,
}

impl ChessBoard {
    pub fn new(size: u16) -> Self {
        Self {
            size,
            background_color: Color::from_rgb8(0x15, 0x88, 0xC4),
            white_cell_color: Color::from_rgb8(0xFF, 0xDE, 0xAD),
            black_cell_color: Color::from_rgb8(0xCD, 0x85, 0x3F),
            //white_turn: true,
        }
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for ChessBoard
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Fixed(self.size as f32)
    }

    fn height(&self) -> Length {
        Length::Fixed(self.size as f32)
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        layout::Node::new(Size::new(self.size as f32, self.size as f32))
    }

    fn draw(
        &self,
        _state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let cells_size = (self.size as f32) * 0.111;
        let bounds = layout.bounds();

        // draw background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_color: Color::TRANSPARENT,
                border_width: 0f32,
                border_radius: BorderRadius::default(),
            },
            self.background_color,
        );

        // draw cells
        (0..8).for_each(|row| {
            (0..8).for_each(|col| {
                let is_white_cell = (row + col) % 2 == 0;
                let cell_color = if is_white_cell {
                    self.white_cell_color
                } else {
                    self.black_cell_color
                };
                let x = cells_size * (col as f32 + 0.5) + bounds.x;
                let y = cells_size * (row as f32 + 0.5) + bounds.y;

                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: cells_size,
                            height: cells_size,
                        },
                        border_radius: BorderRadius::default(),
                        border_width: 0f32,
                        border_color: Color::TRANSPARENT,
                    },
                    cell_color,
                );
            });
        });
    }
}

impl<'a, Message, Renderer> From<ChessBoard> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(board: ChessBoard) -> Self {
        Self::new(board)
    }
}

/*
        let size = self.size as f32;
        let cells = self.cells_cache.draw(
            Size {
                width: size,
                height: size,
            },
            |frame| {
                let background = Path::rectangle(Point::new(0f32, 0f32), frame.size());
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

        let player_turn = self.turn_cache.draw(
            Size {
                width: size,
                height: size,
            },
            |frame| {
                let cells_size = (self.size as f32) * 0.111;
                let location = cells_size * 8.75;
                let circle = Path::circle(Point::new(location, location), cells_size * 0.25);
                frame.fill(
                    &circle,
                    if self.white_turn {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    },
                );
            },
        );
*/
