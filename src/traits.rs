/* Imports */
use crate::board::Board;

/// Pre-defined functions for all chess pieces
pub trait ChessPiece {
    /// If a chess piece can move to tile, the `to`
    /// param is a tuple of two signed integers 
    /// which are the "move request". Will check if
    /// move will cause own color to get checked, if
    /// so, return false
    fn can_move_local(&self, move_: (i8, i8), board: &Board) -> bool;

    /// Get all possible moves on the board, relative to the piece
    fn get_moves_local(&self) -> Vec<(i8, i8)>;

    /// Get color of piece
    fn is_white(&self) -> bool;
}
