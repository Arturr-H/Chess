/* Imports */
use crate::traits::ChessPiece;

/* Structs */
pub struct Board<'a> {
    /* All chess pieces on the board - white and black */
    pieces: Vec<Vec<&'a dyn ChessPiece>>,
}

/* Method implementations */
impl<'a> Board<'a> {
    /* Constructor */
    pub fn new() -> Self {
        Self::default()
    }

    /* Getters */
    pub fn get(&self, x: i8, y: i8) -> Option<&'a dyn ChessPiece> {
        if x.is_positive() && y.is_positive() {
            Some(
                *self.pieces.get(y as usize)?.get(x as usize)?
            )
        }else {
            None
        }
    }
    pub fn get_mut(&mut self, x: i8, y: i8) -> Option<&'a dyn ChessPiece> {
        if x.is_positive() && y.is_positive() {
            Some(
                *self.pieces.get_mut(y as usize)?.get_mut(x as usize)?
            )
        }else {
            None
        }
    }
}

#[allow(unreachable_code)]
impl<'a> Default for Board<'a> {
    fn default() -> Self {
        Self {
            pieces: vec![vec![panic!(); 8]; 8]
        }
    }
}
