/* Imports */
use std::fmt::Debug;
use serde_derive::{ Serialize };
use crate::{
    traits::PieceMethods,
    piece::{ Color, Piece },
    bundle::{ rook::Rook, knight::Knight, bishop::Bishop, queen::Queen, king::King, pawn::Pawn }
};

/* Structs */
#[derive(Clone, Serialize)]
pub struct Board {
    /* All chess pieces on the board - white and black */
    pieces: Vec<Vec<Tile>>,

    /* Which player's turn it is */
    turn: Color
}

/* Board tile */
#[derive(Clone, Copy, Serialize)]
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
    pub fn move_tile(&mut self, original_x: i8, original_y: i8, x: i8, y: i8, mut tile: Tile) -> Result<(), ()> {

        /* If the piece is a pawn, set its status to `has_moved` */
        if let Tile::Piece(Piece::Pawn(pawn)) = tile {
            tile = Tile::Piece(
                Piece::Pawn(
                    Pawn { color: pawn.color(), has_moved: true }
                )
            )
        }

        /* Move and replace tile */
        if x >= 0 && y >= 0 && x < 8 && y < 8 {
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

                /*
                    Copy self (the board) and make the requested move
                    If the move results in check for the mover, then
                    return err that move is illegal
                */
                let mut board_copy = self.clone();
                match board_copy.move_tile(piece.0, piece.1, to.0, to.1, Tile::Piece(move_piece)) {
                    Ok(_) => {
                        if board_copy.is_in_check(board_copy.turn()) {
                            return Err("Illegal move; you are in check")
                        }else {

                            /* Move the actual piece on the actual board */
                            match self.move_tile(piece.0, piece.1, to.0, to.1, Tile::Piece(move_piece)) {
                                Ok(_) => (),
                                Err(_) => return Err("Location out of bounds")
                            };
                        }
                    },
                    Err(_) => return Err("Location out of bounds")
                };

                Ok(())
            }else {
                Err("Can't move there!")
            }
        }else {
            Err("Can't move an empty piece")
        }
    }

    /// Get whose turn it is
    pub fn turn(&self) -> Color {
        self.turn
    }

    /// Toggle turn
    pub fn toggle_turn(&mut self) -> () {
        match self.turn {
            Color::Black => self.turn = Color::White,
            Color::White => self.turn = Color::Black,
        }
    }

    /// Check if color is checkmated
    pub fn is_checkmated(&self, color: Color) -> bool {
        if self.is_in_check(color) {

            /* Iterate over all pieces of the own color, and
                check if any piece can move to prevent checkmate */
            for (y, row) in self.pieces.iter().enumerate() {
                for (x, piece) in row.iter().enumerate() {
                        if let Tile::Piece(p) = piece {
                            /* Only grab the pieces with the same color */
                            if p.color().is_white() == color.is_white() {
                                let (x, y) = (x as i8, y as i8);

                                /* Check every possible move */
                                'inner: for (add_x, add_y) in p.methods().get_moves_local((x, y), &self) {

                                    /* Clone the board, make the move, and later
                                        check if that move prevented checkmate */
                                    let mut board_clone = self.clone();
                                    match board_clone.move_piece_to_coordinate((x, y), (x + add_x, y + add_y)) {
                                        Ok(_) => (),
                                        Err(_) => continue 'inner,
                                    };
                                    
                                    if !board_clone.is_in_check(color) {
                                        return false
                                    }
                                };

                                /* If it's a pawn, check the pawns "side-step" moves */
                                if let Piece::Pawn(pawn) = p {
                                    let moves =
                                        if pawn.color().is_white() 
                                            { &[(-1, -1), (1, -1)] }
                                        else
                                            { &[(-1, 1), (1, 1)] };

                                    'inner: for (add_x, add_y) in moves {
                                        /* Clone the board, make the move, and later
                                            check if that move prevented checkmate */
                                        let mut board_clone = self.clone();
                                        match board_clone.move_piece_to_coordinate((x, y), (x + add_x, y + add_y)) {
                                            Ok(_) => (),
                                            Err(_) => continue 'inner,
                                        };
                                        
                                        if !board_clone.is_in_check(color) {
                                            return false
                                        }
                                    }
                                }
                            };
                        };
                    }
            }

            true
        }
        
        /* Not in check - not checkmated */
        else {
            false
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

        for (index, row) in self.pieces.iter().enumerate() {
            board.push_str(&index.to_string());
            board.push_str(" │ ");
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

        board.push_str("  └────────────────\n");
        board.push_str("    0 1 2 3 4 5 6 7\n");

        write!(f, "{}", board)
    }
}
