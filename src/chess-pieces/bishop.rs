/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece }, board::Board };
use serde_derive::Serialize;
use super::utils::{iterate_look_for_check, get_possible_moves_in_direction};

/* Bishop */
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Bishop {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Bishop {

    /* All possible moves for bishop */
    fn get_moves_local(&self, position: (i8, i8), board: &Board) -> Vec<(i8, i8)> {
        let mut end = Vec::new();

        /* Top Right */
        end.extend(&get_possible_moves_in_direction(board, position, (1, 1)));
        
        /* Bottom Right */
        end.extend(&get_possible_moves_in_direction(board, position, (1, -1)));
        
        /* Bottom Left */
        end.extend(&get_possible_moves_in_direction(board, position, (-1, -1)));
        
        /* Top Left */
        end.extend(&get_possible_moves_in_direction(board, position, (-1, 1)));

        end
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Bishop(Self { color })
    }

    /* If is checking opposing king */
    fn is_checking_king(&self, color_of_king: Color, x: i8, y: i8, board: &crate::board::Board) -> bool {
        let directions = &[
            /* Diagonal */
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];

        if iterate_look_for_check(x, y, board, color_of_king, directions) {
            true
        }else {
            false
        }
    }

    /* Getters */
    fn color(&self) -> Color { self.color }
}
