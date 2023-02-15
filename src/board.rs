/* Imports */
use std::fmt::Debug;
use crate::{
    traits::PieceMethods,
    piece::{ Color, Piece },
    bundle::{ rook::Rook, knight::Horse, bishop::Bishop, queen::Queen, king::King, pawn::Pawn }
};

/* Structs */
pub struct Board {
    /* All chess pieces on the board - white and black */
    pieces: Vec<Vec<Option<Piece>>>,
}

/* Method implementations */
impl Board {
    /* Constructor */
    pub fn new() -> Self {
        Self::default()
    }

    /* Getters */
    pub fn get(&self, x: i8, y: i8) -> Option<Piece> {
        if x >= 0 && y >= 0 {
            *self.pieces.get(y as usize)?.get(x as usize)?
        }else {
            None
        }
    }
    pub fn get_mut(&mut self, x: i8, y: i8) -> Option<&mut Piece> {
        if x.is_positive() && y.is_positive() {
            self.pieces.get_mut(y as usize)?.get_mut(x as usize)?.as_mut()
        }else {
            None
        }
    }
}

#[allow(unreachable_code)]
impl Default for Board {
    fn default() -> Self {
        let w = Color::White;
        let b = Color::Black;

        Self {
            pieces: vec![
                vec![ Some(Rook::new(b)), Some(Horse::new(b)), Some(Bishop::new(b)), Some(Queen::new(b)), Some(King::new(b)), Some(Bishop::new(b)), Some(Horse::new(b)), Some(Rook::new(b)) ],
                vec![ Some(Pawn::new(b)); 8 ],
                vec![ None; 8 ],
                vec![ None; 8 ],
                vec![ None; 8 ],
                vec![ None; 8 ],
                vec![ Some(Pawn::new(w)); 8 ],
                vec![ Some(Rook::new(w)), Some(Horse::new(w)), Some(Bishop::new(w)), Some(Queen::new(w)), Some(King::new(w)), Some(Bishop::new(w)), Some(Horse::new(w)), Some(Rook::new(w)) ],
            ]
        }
    }
}

/* Debug impl */
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // R = Rook, H = Horse, B = Bishop, Q = Queen, K = King, P = Pawn, # = Empty
        let mut board = String::new();

        for row in self.pieces.iter() {
            for piece in row.iter() {
                board.push_str(match piece {
                    Some(p) => match p {
                        Piece::Rook(_) => "R ",
                        Piece::Horse(_) => "H ",
                        Piece::Bishop(_) => "B ",
                        Piece::Queen(_) => "Q ",
                        Piece::King(_) => "K ",
                        Piece::Pawn(_) => "P ",
                    },
                    None => "# ",
                });
            }
            board.push_str("\n");
        }

        write!(f, "{}", board)
    }
}
