/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece, invert_local_moves } };

/* Pawn */
#[derive(Clone, Copy, Debug)]
pub struct Pawn {
    pub color: Color
}

/* Constants */
const LOCAL_MOVES:&[(i8, i8)] = &[(0, 1)];

/* Method implementations */
impl PieceMethods for Pawn {
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        if self.color() == Color::White {
            invert_local_moves(LOCAL_MOVES)
        }else {
            LOCAL_MOVES.to_vec()
        }
    }
    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Pawn(Self { color })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }
}
