/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece } };
use super::{ bishop::{self, Bishop}, rook::{self, Rook} };

/* Queen */
#[derive(Clone, Copy, Debug)]
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

    /* Getters */
    fn color(&self) -> Color { self.color }
}
