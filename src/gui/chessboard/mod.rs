mod drawing_helper;
use drawing_helper::DrawingHelper;

mod pieces_vectors;
use pieces_vectors::PiecesVectors;

mod mouse_handler;
use mouse_handler::MouseHandler;

mod utils;

use iced::event::Status;
use iced::Event::Mouse;
use iced::{Color, Element, Font, Length, Point, Rectangle, Size};
use iced_native::{layout, mouse, renderer, svg, text, Widget};

use pleco::{Board, Piece};

#[derive(Clone)]
struct DragAndDropData {
    start_file: i8,
    start_rank: i8,
    end_file: i8,
    end_rank: i8,
    moved_piece: Piece,
}

#[derive(Clone)]
pub struct ChessBoard {
    size: u16,
    background_color: Color,
    white_cell_color: Color,
    black_cell_color: Color,
    dnd_start_cell_color: Color,
    dnd_end_cell_color: Color,
    dnd_cross_cells_color: Color,
    text_color: Color,
    pieces_images: PiecesVectors,
    logic: Board,
    reversed: bool,
    mouse_x: f32,
    mouse_y: f32,
    drag_and_drop_data: Option<DragAndDropData>,
}

impl ChessBoard {
    pub fn new(size: u16) -> Self {
        Self {
            size,
            background_color: Color::from_rgb8(0x15, 0x88, 0xC4),
            white_cell_color: Color::from_rgb8(0xFF, 0xDE, 0xAD),
            black_cell_color: Color::from_rgb8(0xCD, 0x85, 0x3F),
            dnd_start_cell_color: Color::from_rgb8(0xDE, 0x18, 0x21),
            dnd_end_cell_color: Color::from_rgb8(0x62, 0xC7, 0x39),
            dnd_cross_cells_color: Color::from_rgb8(0x81, 0x44, 0xBD),
            text_color: Color::from_rgb8(0xFF, 0xFF, 0x00),
            pieces_images: PiecesVectors::new(),
            logic: Board::default(),
            reversed: false,
            drag_and_drop_data: None,
            mouse_x: f32::INFINITY,
            mouse_y: f32::INFINITY,
        }
    }

    pub fn toggle_orientation(&mut self) {
        self.reversed = !self.reversed;
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for ChessBoard
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font> + svg::Renderer,
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
        let bounds = layout.bounds();

        DrawingHelper::draw_background(self, renderer, bounds);
        DrawingHelper::draw_cells(self, renderer, bounds);
        DrawingHelper::draw_coordinates(self, renderer, bounds);
        DrawingHelper::draw_player_turn(self, renderer, bounds);
        DrawingHelper::draw_pieces(self, renderer, bounds);
        DrawingHelper::draw_moved_piece(self, renderer, bounds);
    }

    fn on_event(
        &mut self,
        _state: &mut iced_native::widget::Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        _cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        _shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        let bounds = layout.bounds();
        match event {
            Mouse(event) => match event {
                mouse::Event::ButtonPressed(button) => match button {
                    mouse::Button::Left => {
                        MouseHandler::handle_left_button_pressed(self);
                        Status::Captured
                    }
                    _ => Status::Ignored,
                },
                mouse::Event::ButtonReleased(button) => match button {
                    mouse::Button::Left => {
                        MouseHandler::handle_left_button_released(self);
                        Status::Captured
                    }
                    _ => Status::Ignored,
                },
                mouse::Event::CursorMoved { position } => {
                    let x = position.x - bounds.x;
                    let y = position.y - bounds.y;
                    self.mouse_x = x;
                    self.mouse_y = y;
                    MouseHandler::handle_mouse_moved(self);
                    Status::Captured
                }
                _ => Status::Ignored,
            },
            _ => Status::Ignored,
        }
    }
}

impl<'a, Message, Renderer> From<ChessBoard> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font> + svg::Renderer,
{
    fn from(board: ChessBoard) -> Self {
        Self::new(board)
    }
}
