/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece } };
use super::{ bishop::Bishop, rook::Rook, utils::iterate_look_for_check };
use serde_derive::Serialize;

/* Queen */
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Queen {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Queen {

    /* All possible moves for queen (DOES NOT NEED TO BE INVERTED) */
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        /* Combine rook and bishop moves (super smart) */
        vec![
            Bishop::get_moves_local(&Bishop { color: Color::White }),
            Rook::get_moves_local(&Rook { color: Color::White })
        ].concat()
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Queen(Self { color })
    }

    /* If is checking king */
    fn is_checking_king(&self, color_of_king: Color, x: i8, y: i8, board: &crate::board::Board) -> bool {
        let directions = &[
            /* Straight */
            (1, 0), (-1, 0), (0, 1), (0, -1),
            
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
