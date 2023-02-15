/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece } };

/* Knight */
#[derive(Clone, Copy, Debug)]
pub struct Knight {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Knight {

    /* All possible moves for knight */
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
        Piece::Knight(Self { color })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }
}
