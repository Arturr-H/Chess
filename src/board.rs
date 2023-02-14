/* Imports */
use crate::traits::ChessPiece;

/* Structs */
pub struct Board<'a> {
    /* All chess pieces on the board - white and black */
    pieces: Vec<&'a dyn ChessPiece>,
}
