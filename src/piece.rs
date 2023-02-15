/* Imports */
use crate::{
    bundle::{
        rook::Rook,
        knight::Horse,
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
    Horse(Horse),
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
            Self::Horse(e) => e,
            Self::King(e) => e,
            Self::Queen(e) => e,
            Self::Bishop(e) => e,
            Self::Pawn(e) => e,
        }
    }

    /// Can move local to original piece position
    fn can_move_local(&self, move_: (i8, i8), board: &Board) -> bool {
        self.methods().can_move_local(move_, board)
    }

    /// Get color
    pub fn color(&self) -> Color {
        self.methods().color()
    }
}

/// Piece color
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color { White, Black }
