/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece } };

/* Horse */
#[derive(Clone, Copy, Debug)]
pub struct Horse {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Horse {

    /* All possible moves for horse */
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        vec![
            /* Top */
            (-1, 2), (1, 2),

            /* Right */
            (2, 1), (2, -1),

            /* Bottom */
            (1, -2), (-1, -2),

            /* Left */
            (-2, 1), (-2, -1),
        ]
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Horse(Self { color })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }
}
