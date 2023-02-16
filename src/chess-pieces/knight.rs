/* Imports */
use crate::{ traits::PieceMethods, piece::{ Color, Piece }, board::Tile };

/* Knight */
#[derive(Clone, Copy, Debug)]
pub struct Knight {
    pub color: Color
}

/* Method implementations */
impl PieceMethods for Knight {

    /* All possible moves for knight */
    fn get_moves_local(&self) -> Vec<(i8, i8)> {
        vec![
            /* Top */
            (-1, -2), (1, -2),

            /* Right */
            (2, -1), (2, 1),

            /* Bottom */
            (1, 2), (-1, 2),

            /* Left */
            (-2, -1), (-2, 1),
        ]
    }

    /* Constructor */
    fn new<'a>(color: Color) -> Piece where Self: Sized {
        Piece::Knight(Self { color })
    }

    /* Getters */
    fn color(&self) -> Color { self.color }

    /* If is checking opposing king */
    fn is_checking_king(&self, color_of_king: Color, x: i8, y: i8, board: &crate::board::Board) -> bool {
        let places = &[
            /* Top */
            (x - 1, y - 2), (x + 1, y - 2),

            /* Right */
            (x + 2, y - 1), (x + 2, y + 1),

            /* Bottom */
            (x + 1, y + 2), (x - 1, y + 2),

            /* Left */
            (x - 2, y - 1), (x - 2, y + 1),
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
