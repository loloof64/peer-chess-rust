use pleco::{SQ, Piece};

use crate::gui::chessboard::DragAndDropData;

use super::ChessBoard;
use super::utils::Utils;

pub struct MouseHandler {}

impl MouseHandler {
    pub fn handle_left_button_pressed(board: &mut ChessBoard) {
        let cells_size = (board.size as f32) * 0.111;
        if board.drag_and_drop_data.is_none() {
            let x = board.mouse_x;
            let y = board.mouse_y;

            let cell_col = ((x - cells_size * 0.5f32) / cells_size) as i32;
            let cell_row = ((y - cells_size * 0.5f32) / cells_size) as i32;

            let in_cell_bounds = cell_col >= 0 && cell_col <= 7 && cell_row >= 0 && cell_row <= 7;

            if in_cell_bounds {
                let cell_col = cell_col as i8;
                let cell_row = cell_row as i8;

                let start_file = if board.reversed {
                    7 - cell_col
                } else {
                    cell_col
                };
                let start_rank = if board.reversed {
                    cell_row
                } else {
                    7 - cell_row
                };

                let pleco_file = Utils::coord_file_to_pleco_file(start_file as i32);
                let pleco_rank = Utils::coord_rank_to_pleco_rank(start_rank as i32);
                let square = SQ::make(pleco_file, pleco_rank);
                let moved_piece = board.logic.piece_at_sq(square);

                if moved_piece != Piece::None {
                    board.drag_and_drop_data = Some(DragAndDropData {
                        start_file,
                        start_rank,
                        end_file: start_file,
                        end_rank: start_rank,
                        moved_piece,
                    });
                }
            }
        }
    }

    pub fn handle_left_button_released(board: &mut ChessBoard) {
        if board.drag_and_drop_data.is_some() {
            let dnd_data = board.drag_and_drop_data.clone().unwrap();
            let end_file = dnd_data.end_file;
            let end_rank = dnd_data.end_rank;
            
            board.drag_and_drop_data = None;
        }
    }

    pub fn handle_mouse_moved(board: &mut ChessBoard) {
        let cells_size = (board.size as f32) * 0.111;
        if board.drag_and_drop_data.is_some() {
            let x = board.mouse_x;
            let y = board.mouse_y;

            let cell_col = ((x - cells_size * 0.5f32) / cells_size) as i32;
            let cell_row = ((y - cells_size * 0.5f32) / cells_size) as i32;

            let end_file = (if board.reversed {7-cell_col} else {cell_col}) as i8;
            let end_rank = (if board.reversed {cell_row} else {7-cell_row}) as i8;

            let mut dnd_data = board.drag_and_drop_data.clone().unwrap();

            dnd_data.end_file = end_file;
            dnd_data.end_rank = end_rank;

            board.drag_and_drop_data = Some(dnd_data);
        }
    }
}
