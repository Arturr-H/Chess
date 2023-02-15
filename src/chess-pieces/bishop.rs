/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece } };

/* Bishop */
#[derive(Clone, Copy, Debug)]
pub struct Bishop {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Bishop {

    /* All possible moves for bishop */
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        vec![
            /* Top Right */
            (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8),

            /* Bottom Right */
            (1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7), (8, -8),

            /* Bottom Left */
            (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7), (-8, -8),

            /* Top Left */
            (-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7), (-8, 8)
        ]
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Bishop(Self { color })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }
}
