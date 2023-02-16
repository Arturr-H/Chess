/* Imports */
use std::fmt::Debug;
use crate::{
    traits::PieceMethods,
    piece::{ Color, Piece },
    bundle::{ rook::Rook, knight::Knight, bishop::Bishop, queen::Queen, king::King, pawn::Pawn }
};

/* Structs */
pub struct Board {
    /* All chess pieces on the board - white and black */
    pieces: Vec<Vec<Tile>>,

    /* Which player's turn it is */
    turn: Color
}

/* Board tile */
#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Piece(Piece)
}

/* Method implementations */
impl Board {
    /* Constructor */
    pub fn new() -> Self {
        Self::default()
    }

    /* Getters */
    pub fn get(&self, x: i8, y: i8) -> Tile {
        if x >= 0 && y >= 0 {
            match match self.pieces.get(y as usize) {
                Some(e) => e.clone(),
                None => vec![Tile::Empty]
            }.get(x as usize) {
                Some(e) => *e,
                None => Tile::Empty
            }
        }else {
            Tile::Empty
        }
    }

    /// Will replace the tile the piece moves 
    /// from and then performs the move if possible
    pub fn move_tile(&mut self, original_x: i8, original_y: i8, x: i8, y: i8, tile: Tile) -> Result<(), ()> {
        if x >= 0 && y >= 0 {
            self.pieces[original_y as usize][original_x as usize] = Tile::Empty;
            self.pieces[y as usize][x as usize] = tile;
            Ok(())
        }else {
            Err(())
        }
    }

    /// Look if `color` is in check
    pub fn is_in_check(&self, color: Color) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let tile = self.get(x, y);

                match tile {
                    Tile::Piece(piece) => {
                        /* 
                            We want to look for if `color` is in check, 
                            therefore we scan the opposing colors' pieces
                            and check wether they are checking the `color`s king
                        */
                        if piece.color() != color && piece.methods().is_checking_king(color, x, y, &self) {
                            return true;
                        }
                    },
                    Tile::Empty => ()
                };
            }
        }

        false
    }

    /// Move piece, return Err() if didn't succeed
    pub fn move_piece_to_coordinate(&mut self, piece: (i8, i8), to: (i8, i8)) -> Result<(), &str> {
        /* Checks */
        if piece == to { return Err("Can't move to same place") };
        let move_piece = self.get(piece.0, piece.1);
        if let Tile::Piece(move_piece) = move_piece {
            if move_piece.color() != self.turn { return Err("Not right players turn") };
            let from = (piece.0, piece.1);

            /* Check if can move to place */
            if move_piece.can_move_local(from, to, &self) {
                self.move_tile(piece.0, piece.1, to.0, to.1, Tile::Piece(move_piece)).unwrap();
                Ok(())
            }else {
                Err("Can't move there!")
            }
        }else {
            Err("No piece will move")
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
                vec![ Tile::Piece(Rook::new(b)), Tile::Piece(Knight::new(b)), Tile::Piece(Bishop::new(b)), Tile::Piece(Queen::new(b)), Tile::Piece(King::new(b)), Tile::Piece(Bishop::new(b)), Tile::Piece(Knight::new(b)), Tile::Piece(Rook::new(b)) ],
                vec![ Tile::Piece(Pawn::new(b)); 8 ],
                vec![ Tile::Empty; 8 ],
                vec![ Tile::Empty; 8 ],
                vec![ Tile::Empty; 8 ],
                vec![ Tile::Empty; 8 ],
                vec![ Tile::Piece(Pawn::new(w)); 8 ],
                vec![ Tile::Piece(Rook::new(w)), Tile::Piece(Knight::new(w)), Tile::Piece(Bishop::new(w)), Tile::Piece(Queen::new(w)), Tile::Piece(King::new(w)), Tile::Piece(Bishop::new(w)), Tile::Piece(Knight::new(w)), Tile::Piece(Rook::new(w)) ],
            ],
            turn: Color::White
        }
    }
}
impl From<Vec<Vec<Tile>>> for Board {
    fn from(value: Vec<Vec<Tile>>) -> Self {
        Self { pieces: value, turn: Color::White }
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
                    Tile::Piece(p) => match p {
                        Piece::Rook(_) => "R ",
                        Piece::Knight(_) => "H ",
                        Piece::Bishop(_) => "B ",
                        Piece::Queen(_) => "Q ",
                        Piece::King(_) => "K ",
                        Piece::Pawn(_) => "P ",
                    },
                    Tile::Empty => "# ",
                });
            }
            board.push_str("\n");
        }

        write!(f, "{}", board)
    }
}
