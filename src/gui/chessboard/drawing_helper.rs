use std::marker::PhantomData;

use super::utils::Utils;

use super::ChessBoard;

use iced::{Color, Font, Rectangle};

use iced_native::renderer::BorderRadius;
use iced_native::svg::Handle;
use iced_native::text::Text;
use iced_native::{renderer, svg, text};

use pleco::{Piece, Player, SQ};

pub struct DrawingHelper<Renderer>
where
    Renderer: renderer::Renderer + text::Renderer<Font = Font> + svg::Renderer,
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
            let x =
                cells_size * ((if board.reversed { 7 - col } else { col }) as f32 + 1.0) + bounds.x;
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
            let y =
                cells_size * (if board.reversed { 7 - row } else { row } as f32 + 1.0) + bounds.y;

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
                let pleco_file = Utils::coord_file_to_pleco_file(file);
                let pleco_rank = Utils::coord_rank_to_pleco_rank(rank);
                let piece = board.logic.piece_at_sq(SQ::make(pleco_file, pleco_rank));
                let piece_image_handle =
                    DrawingHelper::<Renderer>::pleco_piece_to_image_handle(board, piece);
                if let Some(piece_image_handle) = piece_image_handle {
                    let cell_bounds = Rectangle {
                        x: cells_size
                            * (if board.reversed {
                                7.5f32 - file as f32
                            } else {
                                0.5f32 + file as f32
                            })
                            + bounds.x,
                        y: cells_size
                            * (if board.reversed {
                                0.5f32 + rank as f32
                            } else {
                                7.5f32 - rank as f32
                            })
                            + bounds.y,
                        width: cells_size,
                        height: cells_size,
                    };
                    renderer.draw(piece_image_handle, None, cell_bounds);
                }
            });
        });
    }

    pub fn draw_moved_piece(board: &ChessBoard, renderer: &mut Renderer, bounds: Rectangle) {
        let cells_size = (board.size as f32) * 0.111;

        if board.drag_and_drop_data.is_some() {
            let piece_image_handle = DrawingHelper::<Renderer>::pleco_piece_to_image_handle(
                board,
                board.drag_and_drop_data.clone().unwrap().moved_piece,
            );
            if let Some(piece_image_handle) = piece_image_handle {
                let cell_bounds = Rectangle {
                    x: board.mouse_x + bounds.x,
                    y: board.mouse_y + bounds.y,
                    width: cells_size,
                    height: cells_size,
                };
                renderer.draw(piece_image_handle, None, cell_bounds);
            }
        }
    }

    fn pleco_piece_to_image_handle(board: &ChessBoard, piece: Piece) -> Option<Handle> {
        match piece {
            Piece::None => None,
            Piece::WhitePawn => Some(board.pieces_images.svg_wp.clone()),
            Piece::WhiteKnight => Some(board.pieces_images.svg_wn.clone()),
            Piece::WhiteBishop => Some(board.pieces_images.svg_wb.clone()),
            Piece::WhiteRook => Some(board.pieces_images.svg_wr.clone()),
            Piece::WhiteQueen => Some(board.pieces_images.svg_wq.clone()),
            Piece::WhiteKing => Some(board.pieces_images.svg_wk.clone()),
            Piece::BlackPawn => Some(board.pieces_images.svg_bp.clone()),
            Piece::BlackKnight => Some(board.pieces_images.svg_bn.clone()),
            Piece::BlackBishop => Some(board.pieces_images.svg_bb.clone()),
            Piece::BlackRook => Some(board.pieces_images.svg_br.clone()),
            Piece::BlackQueen => Some(board.pieces_images.svg_bq.clone()),
            Piece::BlackKing => Some(board.pieces_images.svg_bk.clone()),
        }
    }
}
