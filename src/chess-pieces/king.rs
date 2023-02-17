/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece, invert_local_moves }, board::Tile };
use serde_derive::Serialize;

/* King */
#[derive(Clone, Copy, Debug, Serialize)]
pub struct King {
    pub color: Color
}

/* Constants */
const LOCAL_MOVES:&[(i8, i8)] = &[
    (-1, 1) , (0, 1) , (1, 1),
    (-1, 0) ,          (1, 0),
    (-1, -1), (0, -1), (1, -1)
];

/* Method implementations */
impl PieceMethods for King {

    /* All possible moves for bishop */
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        if self.color() == Color::White {
            LOCAL_MOVES.to_vec()
        }else {
            invert_local_moves(LOCAL_MOVES)
        }
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::King(Self { color })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }

    /* Check if cheking opposing king */
    fn is_checking_king(&self, color_of_king: Color, x: i8, y: i8, board: &crate::board::Board) -> bool {
        let places = &[
            (x-1, y-1), (x, y-1), (x+1, y-1),
            (x-1, y)  ,           (x+1, y),
            (x-1, y+1), (x, y+1), (x+1, y+1)
        ];

        for (new_x, new_y) in places {
            
            /* Check item */
            match board.get(*new_x, *new_y) {
                Tile::Piece(e) => {
                    match e {
                        Piece::King(e) => {
                            if e.color() == color_of_king {
                                return true
                            }
                        },
                        _ => { continue; }
                    }
                },
                Tile::Empty => ()
            }
        }
        
        false
    }
}
