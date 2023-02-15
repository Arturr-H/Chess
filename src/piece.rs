/* Imports */
use crate::{
    bundle::{
        rook::Rook,
        knight::Knight,
        king::King,
        queen::Queen,
        bishop::Bishop,
        pawn::Pawn
    },
    board::Board, traits::PieceMethods
};

/// The pieces
#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Rook(Rook),
    Knight(Knight),
    King(King),
    Queen(Queen),
    Bishop(Bishop),
    Pawn(Pawn),
}

/* Method implementations */
impl Piece {
    /// Get piece methods
    pub fn methods(&self) -> &dyn PieceMethods {
        match self {
            Self::Rook(e) => e,
            Self::Knight(e) => e,
            Self::King(e) => e,
            Self::Queen(e) => e,
            Self::Bishop(e) => e,
            Self::Pawn(e) => e,
        }
    }

    /// Can move local to original piece position
    pub fn can_move_local(&self, from: (i8, i8), to: (i8, i8), board: &Board) -> bool {
        self.methods().can_move_local(from, to, board)
    }

    /// Get color
    pub fn color(&self) -> Color {
        self.methods().color()
    }
}

/* Invert local moves for piece */
pub fn invert_local_moves(moves: &[(i8, i8)]) -> Vec<(i8, i8)> {
    let mut end: Vec<(i8, i8)> = Vec::with_capacity(moves.len());
    for (x, y) in moves {
        end.push((
            x*(-1),
            y*(-1)
        ))
    };

    end
}

/// Piece color
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color { White, Black }
