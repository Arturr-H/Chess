/* Imports */
use crate::board::Board;

/// Piece type
pub enum PieceType {
    White,
    Black,
    Empty
}

/// Method impl
impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PieceType::White, PieceType::White) => true,
            (PieceType::Black, PieceType::Black) => true,
            (PieceType::Empty, PieceType::Empty) => true,
            _ => false,
        }
    }
}

/// Pre-defined functions for all chess pieces
pub trait ChessPiece {
    /// If a chess piece can move to tile, the `to`
    /// param is a tuple of two signed integers 
    /// which are the "move request". Will check if
    /// move will cause own color to get checked, if
    /// so, return false
    fn can_move_local(&self, move_: (i8, i8), board: &Board) -> bool {
        /* If move is physically possible by that piece */
        if self.get_moves_local().contains(&move_) {
            if let Some(piece) = board.get(move_.0, move_.1) {
                let type_ = piece.piece_type();

                // TODO: look out for if king is checked

                if type_ != self.piece_type() {
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
    fn get_moves_local(&self) -> Vec<(i8, i8)>;

    /// Get color of piece
    fn piece_type(&self) -> &PieceType;
}
