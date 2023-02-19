/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece }, board::Board };
use serde_derive::Serialize;
use super::utils::{iterate_look_for_check, get_possible_moves_in_direction};

/* Rook */
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Rook {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Rook {

    /* All possible moves for rook */
    fn get_moves_local(&self, position: (i8, i8), board: &Board) -> Vec<(i8, i8)> {
        let mut end = Vec::new();
        
        /* Top */
        end.extend(&get_possible_moves_in_direction(board, position, (0, 1)));

        /* Bottom */
        end.extend(&get_possible_moves_in_direction(board, position, (0, -1)));

        /* Right */
        end.extend(&get_possible_moves_in_direction(board, position, (1, 0)));

        /* Left */
        end.extend(&get_possible_moves_in_direction(board, position, (-1, 0)));

        end
    }
    
    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Rook(Self { color })
    }

    /* If is checking king */
    fn is_checking_king(&self, color_of_king: Color, x: i8, y: i8, board: &crate::board::Board) -> bool {
        let directions = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

        if iterate_look_for_check(x, y, board, color_of_king, directions) {
            true
        }else {
            false
        }
    }

    /* Getters */
    fn color(&self) -> Color { self.color }
}
