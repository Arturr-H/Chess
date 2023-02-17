/* Imports */
use super::board::Board;
use uuid;
use rand::{self, Rng};

/* Main */
pub struct Game {
    board: Board,

    /* Player uids, they are options because
        player 2 might not be connected - player
        1 creates game and waits for player 2 */
    white: Option<String>,
    black: Option<String>,

    /* Game-ID */
    id: String
}

/* Method implementations */
impl Game {
    /// Create a new `Game` struct. The `player`
    /// parameter is the uuid of the player who
    /// created the game.
    pub fn new(player: String) -> Self {
        let player_1_white = rand::thread_rng().gen_bool(0.5f64);

        /* If player 1 should be white */
        if player_1_white {
            Self {
                board: Board::new(),
                white: Some(player),
                black: None,
                id: uuid::Uuid::new_v4().as_hyphenated().to_string()
            }
        }else {
            Self {
                board: Board::new(),
                white: None,
                black: Some(player),
                id: uuid::Uuid::new_v4().as_hyphenated().to_string()
            }
        }
    }

    /// Append player to empty color, if 
    /// spot is taken, return Err(_)
    pub fn insert_player(&mut self, player: String) -> Result<(), ()> {
        match self.black() {
            Some(_) => {
                match self.white() {
                    Some(_) => Err(()),
                    None => {
                        self.white = Some(player);
                        Ok(())
                    }
                }
            },
            None => {
                self.black = Some(player);
                Ok(())
            }
        }
    }

    /// Returns a boolean wether game is occupied
    pub fn occupied(&self) -> bool {
        match (self.black(), self.white()) {
            (Some(_), Some(_)) => true,
            _ => false
        }
    }

    /* Getters */
    pub fn white(&self) -> Option<&String> { self.white.as_ref() }
    pub fn black(&self) -> Option<&String> { self.black.as_ref() }
    pub fn id(&self) -> &String { &self.id }
    pub fn board(&self) -> &Board { &self.board }
    pub fn board_mut(&mut self) -> &mut Board { &mut self.board }
}
