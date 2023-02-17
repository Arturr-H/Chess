/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece } };
use serde_derive::Serialize;
use super::utils::iterate_look_for_check;

/* Rook */
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Rook {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Rook {

    /* All possible moves for rook */
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        vec![
            /* Top */
            (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),

            /* Right */
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0),

            /* Bottom */
            (0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7), (0, -8),

            /* Left */
            (-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0), (-8, 0)
        ]
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
