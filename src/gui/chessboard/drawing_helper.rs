use std::marker::PhantomData;

use super::ChessBoard;

use iced::{Color, Font, Rectangle};

use iced_native::renderer::BorderRadius;
use iced_native::svg::Handle;
use iced_native::text::Text;
use iced_native::{ renderer, svg, text};

use pleco::{ File, Piece, Rank, SQ, Player};

pub struct DrawingHelper<Renderer>
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font>,
{
    _renderer: PhantomData<Renderer>,
}

impl<Renderer> DrawingHelper<Renderer>
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font> + svg::Renderer,
{
    pub fn draw_background(board: &ChessBoard, renderer: &mut Renderer, bounds: Rectangle) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_color: Color::TRANSPARENT,
                border_width: 0f32,
                border_radius: BorderRadius::default(),
            },
            board.background_color,
        );
    }

    pub fn draw_cells(board: &ChessBoard, renderer: &mut Renderer, bounds: Rectangle) {
        let cells_size = (board.size as f32) * 0.111;
        (0..8).for_each(|row| {
            (0..8).for_each(|col| {
                let is_white_cell = (row + col) % 2 == 0;
                let cell_color = if is_white_cell {
                    board.white_cell_color
                } else {
                    board.black_cell_color
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

    pub fn draw_coordinates(board: &ChessBoard, renderer: &mut Renderer, bounds: Rectangle) {
        let files = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        let ranks = vec!["8", "7", "6", "5", "4", "3", "2", "1"];

        files.into_iter().enumerate().for_each(|(col, file_str)| {
            let cells_size = (board.size as f32) * 0.111;
            let font_size = cells_size * 0.4;
            let x = cells_size * ((if board.reversed {7-col} else {col}) as f32 + 1.0) + bounds.x;
            let y1 = cells_size * 0.25 + bounds.y;
            let y2 = cells_size * 8.75 + bounds.y;

            let text1 = Text {
                content: file_str.into(),
                bounds: Rectangle {
                    x,
                    y: y1,
                    width: font_size,
                    height: font_size,
                },
                color: board.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text1);

            let text2 = Text {
                content: file_str.into(),
                bounds: Rectangle {
                    x,
                    y: y2,
                    width: font_size,
                    height: font_size,
                },
                color: board.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text2);
        });

        ranks.into_iter().enumerate().for_each(|(row, rank_str)| {
            let cells_size = (board.size as f32) * 0.111;
            let font_size = cells_size * 0.4;
            let x1 = cells_size * 0.25 + bounds.x;
            let x2 = cells_size * 8.75 + bounds.x;
            let y = cells_size * (if board.reversed {7-row} else {row} as f32 + 1.0) + bounds.y;

            let text1 = Text {
                content: rank_str.into(),
                bounds: Rectangle {
                    x: x1,
                    y: y,
                    width: font_size,
                    height: font_size,
                },
                color: board.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text1);

            let text2 = Text {
                content: rank_str.into(),
                bounds: Rectangle {
                    x: x2,
                    y,
                    width: font_size,
                    height: font_size,
                },
                color: board.text_color,
                size: font_size,
                font: Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
            };
            renderer.fill_text(text2);
        });
    }

    pub fn draw_player_turn(board: &ChessBoard, renderer: &mut Renderer, bounds: Rectangle) {
        let cells_size = (board.size as f32) * 0.111;
        let x = cells_size * 8.5 + bounds.x;
        let y = cells_size * 8.5 + bounds.y;
        let color = if board.logic.turn() == Player::White {
            Color::WHITE
        } else {
            Color::BLACK
        };
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x,
                    y,
                    width: cells_size * 0.5f32,
                    height: cells_size * 0.5f32,
                },
                border_radius: BorderRadius::from(cells_size * 0.5f32),
                border_width: 0f32,
                border_color: Color::TRANSPARENT,
            },
            color,
        );
    }

    pub fn draw_pieces(board: &ChessBoard, renderer: &mut Renderer, bounds: Rectangle) {
        let cells_size = (board.size as f32) * 0.111;

        (0..8).for_each(|row| {
            (0..8).for_each(|col| {
                let file = col;
                let rank = 7 - row;
                let pleco_file = DrawingHelper::<Renderer>::coord_file_to_pleco_file(file);
                let pleco_rank = DrawingHelper::<Renderer>::coord_rank_to_pleco_rank(rank);
                let piece = board.logic.piece_at_sq(SQ::make(pleco_file, pleco_rank));
                let piece_image_handle =
                    DrawingHelper::<Renderer>::pleco_piece_to_image_handle(board, piece);
                if piece_image_handle.is_some() {
                    let cell_bounds = Rectangle {
                        x: cells_size * (if board.reversed {7.5f32 - file as f32} else  {0.5f32 + file as f32}) + bounds.x,
                        y: cells_size * (if board.reversed {0.5f32 + rank as f32} else {7.5f32 - rank as f32}) + bounds.y,
                        width: cells_size,
                        height: cells_size,
                    };
                    renderer.draw(piece_image_handle.unwrap().clone(), None, cell_bounds);
                }
            });
        });
    }

    fn coord_file_to_pleco_file(input: i32) -> File {
        let constrained_input = input.max(0).min(7);
        match constrained_input {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => File::A,
        }
    }

    fn coord_rank_to_pleco_rank(input: i32) -> Rank {
        let constrained_input = input.max(0).min(7);
        match constrained_input {
            0 => Rank::R1,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            _ => Rank::R1,
        }
    }

    pub fn pleco_piece_to_image_handle(board: &ChessBoard, piece: Piece) -> Option<Handle> {
        match piece {
            Piece::None => None,
            Piece::WhitePawn => Some(board.svg_wp.clone()),
            Piece::WhiteKnight => Some(board.svg_wn.clone()),
            Piece::WhiteBishop => Some(board.svg_wb.clone()),
            Piece::WhiteRook => Some(board.svg_wr.clone()),
            Piece::WhiteQueen => Some(board.svg_wq.clone()),
            Piece::WhiteKing => Some(board.svg_wk.clone()),
            Piece::BlackPawn => Some(board.svg_bp.clone()),
            Piece::BlackKnight => Some(board.svg_bn.clone()),
            Piece::BlackBishop => Some(board.svg_bb.clone()),
            Piece::BlackRook => Some(board.svg_br.clone()),
            Piece::BlackQueen => Some(board.svg_bq.clone()),
            Piece::BlackKing => Some(board.svg_bk.clone()),
        }
    }
}
