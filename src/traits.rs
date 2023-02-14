/* Imports */
use crate::board::Board;

/// Pre-defined functions for all chess pieces
pub trait ChessPiece {
    /// If a chess piece can move to tile, the `to`
    /// param is a tuple of two signed integers 
    /// which are the "move request". Will check if
    /// move will cause own color to get checked, if
    /// so, return false
    fn can_move(&self, move_: (i8, i8), board: &Board) -> bool;
}
