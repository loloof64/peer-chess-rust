mod drawing_helper;
use drawing_helper::DrawingHelper;

use iced::{Color, Element, Font, Length, Point, Rectangle, Size};

use iced_native::svg::Handle;
use iced_native::{layout, renderer, svg, text, Widget};

use pleco::Board;

#[derive(Clone)]
pub struct ChessBoard {
    size: u16,
    background_color: Color,
    white_cell_color: Color,
    black_cell_color: Color,
    text_color: Color,
    svg_wp: Handle,
    svg_wn: Handle,
    svg_wb: Handle,
    svg_wr: Handle,
    svg_wq: Handle,
    svg_wk: Handle,
    svg_bp: Handle,
    svg_bn: Handle,
    svg_bb: Handle,
    svg_br: Handle,
    svg_bq: Handle,
    svg_bk: Handle,
    logic: Board,
    reversed: bool,
}

impl ChessBoard {
    pub fn new(size: u16) -> Self {
        let svg_wp = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_plt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wn = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_nlt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wb = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_blt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wr = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_rlt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wq = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_qlt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wk = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_klt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bp = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_pdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bn = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_ndt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bb = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_bdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_br = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_rdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bq = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_qdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bk = svg::Handle::from_path(format!(
            "{}/resources/chess_vectors/Chess_kdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        Self {
            size,
            background_color: Color::from_rgb8(0x15, 0x88, 0xC4),
            white_cell_color: Color::from_rgb8(0xFF, 0xDE, 0xAD),
            black_cell_color: Color::from_rgb8(0xCD, 0x85, 0x3F),
            text_color: Color::from_rgb8(0xFF, 0xFF, 0x00),
            svg_wp,
            svg_wn,
            svg_wb,
            svg_wr,
            svg_wq,
            svg_wk,
            svg_bp,
            svg_bn,
            svg_bb,
            svg_br,
            svg_bq,
            svg_bk,
            logic: Board::default(),
            reversed: false,
        }
    }

    pub fn toggle_orientation(&mut self) {
        self.reversed = ! self.reversed;
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
