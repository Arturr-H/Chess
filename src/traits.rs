/* Imports */
use crate::{
    board::{Board, Tile},
    piece::{Color, Piece}
};

/// Pre-defined functions for all chess pieces
pub trait PieceMethods {
    /// If a chess piece can move to tile, the `to`
    /// param is a tuple of two signed integers 
    /// which are the "move request". Will check if
    /// move will cause own color to get checked, if
    /// so, return false
    fn can_move_local(&self, from: (i8, i8), to: (i8, i8), board: &Board) -> bool {
        /* Check if `to` is in local move array */
        for (add_x, add_y) in self.get_moves_local() {

            /* If the move seems possible */
            if (from.0 + add_x, from.1 + add_y) == to {

                /* Spot with a piece */
                if let Tile::Piece(piece) = board.get(to.0, to.1) {
                    
                    /* We can't take our own pieces */
                    if piece.color() != self.color() {
                        return true
                    }else {
                        continue;
                    }
                }
                
                /* Empty spot */
                else {
                    return true
                }
            }else {
                continue;
            }
        }

        false
    }

    /// Get all possible moves on the board, relative to the piece
    #[allow(unused_variables)]
    fn is_checking_king(&self, color_of_king: Color, x: i8, y: i8, board: &Board) -> bool { false }

    /// Get all possible moves on the board, relative to the piece
    fn get_moves_local(&self, board: &Board) -> Vec<(i8, i8)>;

    /// Get the color
    fn color(&self) -> Color;

    /// Constructor
    fn new<'a>(color: Color) -> Piece where Self: Sized;
}
