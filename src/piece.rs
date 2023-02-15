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
#[derive(Clone, Copy)]
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
    pub fn get_generic_inner(&self) -> &dyn PieceMethods {
        match self {
            Self::Rook(e) => e,
            Self::Horse(e) => e,
            Self::King(e) => e,
            Self::Queen(e) => e,
            Self::Bishop(e) => e,
            Self::Pawn(e) => e,
        }
    }

    /// Get color
    pub fn color(&self) -> Color {
        self.get_generic_inner().color()
    }
}

/// Piece color
#[derive(Clone, Copy, PartialEq)]
pub enum Color { White, Black }
