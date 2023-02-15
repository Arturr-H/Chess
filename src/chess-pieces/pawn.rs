/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece, invert_local_moves }, board::Board };

/* Pawn */
#[derive(Clone, Copy, Debug)]
pub struct Pawn {
    pub color: Color
}

/* Constants */
const LOCAL_MOVES:&[(i8, i8)] = &[(0, 1)];

/* Method implementations */
impl PieceMethods for Pawn {
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        if self.color() == Color::White {
            invert_local_moves(LOCAL_MOVES)
        }else {
            LOCAL_MOVES.to_vec()
        }
    }
    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Pawn(Self { color })
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
    if let Some(piece) = board.get(x, y) {
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

