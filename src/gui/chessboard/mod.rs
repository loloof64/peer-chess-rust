use iced::{Color, Element, Length, Point, Rectangle, Size, Font};

use iced_native::renderer::BorderRadius;
use iced_native::text::Text;
use iced_native::{layout, renderer, text, Widget};

#[derive(Clone, Copy)]
pub struct ChessBoard {
    size: u16,
    background_color: Color,
    white_cell_color: Color,
    black_cell_color: Color,
    text_color: Color,
    //white_turn: bool,
}

impl ChessBoard {
    pub fn new(size: u16) -> Self {
        Self {
            size,
            background_color: Color::from_rgb8(0x15, 0x88, 0xC4),
            white_cell_color: Color::from_rgb8(0xFF, 0xDE, 0xAD),
            black_cell_color: Color::from_rgb8(0xCD, 0x85, 0x3F),
            text_color: Color::from_rgb8(0xFF, 0xFF, 0x00),
            //white_turn: true,
        }
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for ChessBoard
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
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

        let files = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        let ranks = vec!["8", "7", "6", "5", "4", "3", "2", "1"];

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

        // draw coordinates
        files.into_iter().enumerate().for_each(|(col, file)| {
            let cells_size = (self.size as f32) * 0.111;
            let font_size = cells_size * 0.4;
            let x = cells_size * (col as f32 + 1.0) + bounds.x;
            let y1 = cells_size * 0.25 + bounds.y;
            let y2 = cells_size * 8.75 + bounds.y;

            let text1 = Text {
                content: file.into(),
                bounds: Rectangle { x, y: y1, width: font_size, height: font_size },
                color: self.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text1);

            let text2 = Text {
                content: file.into(),
                bounds: Rectangle { x, y: y2, width: font_size, height: font_size },
                color: self.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text2);
        });

        ranks.into_iter().enumerate().for_each(|(row, rank)| {
            let cells_size = (self.size as f32) * 0.111;
            let font_size = cells_size * 0.4;
            let x1 = cells_size * 0.25 + bounds.x;
            let x2 = cells_size * 8.75 + bounds.x;
            let y = cells_size * (row as f32 + 1.0) + bounds.y;

            let text1 = Text {
                content: rank.into(),
                bounds: Rectangle { x: x1, y: y, width: font_size, height: font_size },
                color: self.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text1);

            let text2 = Text {
                content: rank.into(),
                bounds: Rectangle { x: x2, y, width: font_size, height: font_size },
                color: self.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text2);
        });
    }
}

impl<'a, Message, Renderer> From<ChessBoard> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
{
    fn from(board: ChessBoard) -> Self {
        Self::new(board)
    }
}

/*
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
