use iced::widget::canvas::{self, Cursor, Event, Geometry};
use iced::widget::canvas::{Cache, Canvas, Path};
use iced::{event, mouse, Color, Element, Length, Rectangle, Theme, Size, Point};

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
}

impl ChessBoardBackground {
    pub fn new(size: u16) -> Self {
        Self {
            size,
            white_cell_color: Color::from_rgb8(0xFF, 0xDE, 0xAD),
            black_cell_color: Color::from_rgb8(0xCD, 0x85, 0x3F),
            cells_cache: Cache::default(),
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
        let half_size = size / 2f32;
        let cells = self.cells_cache.draw(Size { width: size, height: size }, |frame| {
            let center = frame.center();
            let background = Path::rectangle(Point::new(center.x - half_size, center.y - half_size), frame.size());
            frame.fill(&background, Color::from_rgb8(0x00, 0x80, 0xFF));

            (0..8).for_each(|row| {
                (0..8).for_each(|col| {
                    let cells_size = (self.size as f32) * 0.111;
                   let is_white_cell = (row+col) % 2 == 0;
                   let cell_color = if is_white_cell  {self.white_cell_color} else {self.black_cell_color};
                   let x = cells_size * (col as f32 + 0.5);
                   let y = cells_size * (row as f32 + 0.5);

                   let bounds = Path::rectangle(Point::new(x, y), Size::new(cells_size, cells_size));
                   frame.fill(&bounds, cell_color);
                });
            });
        });
        vec![cells]
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
