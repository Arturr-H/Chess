/* Imports */
use crate::{
    piece::{ Piece, Color },
    board::{Board, Tile},
    traits::PieceMethods
};


/* 
    Check all lines in diffrent directions if 
    piece is checking opposing king, works for
    queen, rook and bishop because they have 
    straight lines
*/
pub fn iterate_look_for_check(x: i8, y: i8, board: &Board, color_of_king: Color, directions: &[(i8, i8)]) -> bool {
    'outer: for (add_x, add_y) in directions {
        let mut position: (i8, i8) = (x, y);

        /* Increment position */
        for _ in 0..8 {
            position = (position.0 + add_x, position.1 + add_y);

            /* Check item */
            match board.get(position.0, position.1) {
                Tile::Piece(e) => {
                    match e {
                        Piece::King(e) => {
                            if e.color() == color_of_king {
                                return true
                            }
                        },
                        _ => { continue 'outer }
                    }
                },
                Tile::Empty => ()
            }
        }
    }

    false
}

/*
    This function will get all possible moves in a direction.
    If the rook stands in the middle of the board with a pawn
    infront of it (opposing pawn), it can move as far as
    possible down, left and right. It can only move one move 
    forwards (capturing the pawn).
*/
pub fn get_possible_moves_in_direction(board: &Board, from: (i8, i8), direction: (i8, i8)) -> Vec<(i8, i8)> {
    let mut end = Vec::new();
    let self_color = if let Tile::Piece(e) = board.get(from.0, from.1) 
                    { e.color() } else { panic!("Should not panic") };

    for i in 1..8 {
        let position: (i8, i8) = (
            direction.0*i + from.0,
            direction.1*i + from.1
        );

        /* Check item */
        match board.get(position.0, position.1) {
            Tile::Piece(e) => {
                if e.color() == self_color {
                    break;
                }else {
                    end.push(position);
                    break;
                }
            },
            Tile::Empty => end.push(position)
        }
    };

    end
}
