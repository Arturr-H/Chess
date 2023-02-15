/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece, invert_local_moves } };

/* King */
#[derive(Clone, Copy, Debug)]
pub struct King {
    pub color: Color
}

/* Constants */
const LOCAL_MOVES:&[(i8, i8)] = &[
    (-1, 1) , (0, 1) , (1, 1),
    (-1, 0) ,          (1, 0),
    (-1, -1), (0, -1), (1, -1)
];

/* Method implementations */
impl PieceMethods for King {

    /* All possible moves for bishop */
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        if self.color() == Color::White {
            LOCAL_MOVES.to_vec()
        }else {
            invert_local_moves(LOCAL_MOVES)
        }
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::King(Self { color })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }
}
