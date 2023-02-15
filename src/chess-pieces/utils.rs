/* Imports */
use crate::{
    piece::{ Piece, Color },
    board::Board,
    traits::PieceMethods
};


/* 
    Check all lines in diffrent directions if 
    piece is checking opposing king, works for
    queen, rook and bishop because they have 
    straight lines
*/
pub fn iterate_look_for_check(x: i8, y: i8, board: &Board, color_of_king: Color, directions: &[(i8, i8)]) -> bool {
    for (add_x, add_y) in directions {
        let mut position: (i8, i8) = (x, y);

        /* Increment position */
        for _ in 0..8 {
            position = (position.0 + add_x, position.1 + add_y);

            /* Check item */
            match board.get(position.0, position.1) {
                Some(e) => {
                    match e {
                        Piece::King(e) => {
                            if e.color() == color_of_king {
                                return true
                            }
                        },
                        _ => { break }
                    }
                },
                None => ()
            }
        }
    }

    false
}
