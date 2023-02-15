/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece } };

/* Rook */
#[derive(Clone, Copy, Debug)]
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

    /* Getters */
    fn color(&self) -> Color { self.color }
}
