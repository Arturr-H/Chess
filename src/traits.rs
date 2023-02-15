/* Imports */
use crate::{
    bundle::{
        rook::Rook,
        horse::Horse,
        king::King,
        queen::Queen,
        bishop::Bishop,
        pawn::Pawn
    },
    board::Board, piece::{Color, Piece}
};

/// Pre-defined functions for all chess pieces
pub trait PieceMethods {
    /// If a chess piece can move to tile, the `to`
    /// param is a tuple of two signed integers 
    /// which are the "move request". Will check if
    /// move will cause own color to get checked, if
    /// so, return false
    fn can_move_local(&self, move_: (i8, i8), board: &Board) -> bool where Self: Sized {
        /* If move is physically possible by that piece */
        if Self::get_moves_local().contains(&move_) {
            if let Some(piece) = board.get(move_.0, move_.1) {
                let type_ = piece.color();

                // TODO: look out for if king is checked

                if type_ != self.color() {
                    true
                }else {
                    false
                }
            }else {
                false
            }
        }else {
            false
        }
    }

    /// Get all possible moves on the board, relative to the piece
    fn get_moves_local() -> Vec<(i8, i8)> where Self: Sized;

    /// Get the color
    fn color(&self) -> Color;

    /// Constructor
    fn new<'a>(color: Color) -> Piece where Self: Sized;
}
