/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece, invert_local_moves }, board::{Board, Tile} };
use serde_derive::Serialize;

/* Pawn */
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Pawn {
    pub color: Color,

    /* User for looking if pawn can do the "double" leap or whatever you call that */
    pub has_moved: bool
}

/* Constants */
const LOCAL_MOVES:&[(i8, i8)] = &[(0, 1)];
const LOCAL_MOVES_WITH_LEAP:&[(i8, i8)] = &[(0, 1), (0, 2)];

/* Method implementations */
impl PieceMethods for Pawn {
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        if self.has_moved {
            if self.color() == Color::White {
                invert_local_moves(LOCAL_MOVES)
            }else {
                LOCAL_MOVES.to_vec()
            }
        }else {
            if self.color() == Color::White {
                invert_local_moves(LOCAL_MOVES_WITH_LEAP)
            }else {
                LOCAL_MOVES_WITH_LEAP.to_vec()
            }
        }
    }

    /// If a chess piece can move to tile, the `to`
    /// param is a tuple of two signed integers 
    /// which are the "move request". Will check if
    /// move will cause own color to get checked, if
    /// so, return false
    fn can_move_local(&self, from: (i8, i8), to: (i8, i8), board: &Board) -> bool {
        /* Check if `to` is in local move array */
        for (add_x, add_y) in self.get_moves_local() {

            /* Firstly, if the pawn wants to do the "double leap", we
                need to check if theres a pawn infront blocking the leap.
                If there's a piece blocking, the move is incorrect */
            if add_y == 2 || add_y == -2 {
                if let Tile::Piece(_) = board.get(from.0, from.1 + add_y/2) {
                    continue;
                }
            }

            /* If the move seems possible */
            if (from.0 + add_x, from.1 + add_y) == to {

                /* Pawns can only move forward if there's an empty piece there */
                if let Tile::Empty = board.get(to.0, to.1) {
                    return true
                }
            }else {
                continue;
            }
        }

        /* Check if pawn can do their "side-step" */
        let check = 
            /* If the color is black, pieces move down */
            if self.color().is_black()
                { (from.0 + 1, from.1 + 1) == to || (from.0 - 1, from.1 + 1) == to }

            /* Else pieces move up */
            else 
                { (from.0 + 1, from.1 - 1) == to || (from.0 - 1, from.1 - 1) == to };

        if check {
            /* Can only move if there is a piece there */
            if let Tile::Piece(piece) = board.get(to.0, to.1) {
    
                /* We can't take our own pieces */
                if piece.color() != self.color() {
                    return true
                }
            }
        }

        false
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Pawn(Self { color, has_moved: false })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }

    /* If is checking opposing king */
    fn is_checking_king(&self, color_of_king: Color, x: i8, y: i8, board: &crate::board::Board) -> bool {
        match self.color {
            Color::Black => {
                let check_locations = &[(x - 1, y + 1), (x + 1, y + 1)];
                for (new_x, new_y) in check_locations {
                    if is_checking_king(*new_x, *new_y, board, color_of_king) {
                        return true
                    }
                };

                false
            },
            Color::White => {
                let check_locations = &[(x - 1, y - 1), (x + 1, y - 1)];
                for (new_x, new_y) in check_locations {
                    if is_checking_king(*new_x, *new_y, board, color_of_king) {
                        return true
                    }
                };

                false
            },
        }
    }
}
fn is_checking_king(x: i8, y: i8, board: &Board, color_of_king: Color) -> bool {
    if let Tile::Piece(piece) = board.get(x, y) {
        match piece {
            Piece::King(e) => {
                if e.color() == color_of_king {
                    return true
                }
            },
            _ => ()
        }
    }

    false
}

