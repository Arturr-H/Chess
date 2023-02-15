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
    board::Board, piece::{Color, Piece}
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
                if let Some(piece) = board.get(to.0, to.1) {
                    // TODO: look out for if king is checked & if pieces are blocking
                    
                    /* We can't take our own pieces */
                    if piece.color() != self.color() {
                        return true
                    }else {
                        return false
                    }
                }
                
                /* Empty spot */
                else {
                    return true
                }
            }else {
                return false
            }
        }

        false
    }

    /// Get all possible moves on the board, relative to the piece
    fn get_moves_local(&self) -> Vec<(i8, i8)>;

    /// Get the color
    fn color(&self) -> Color;

    /// Constructor
    fn new<'a>(color: Color) -> Piece where Self: Sized;
}
