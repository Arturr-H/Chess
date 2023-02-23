use std::net::SocketAddr;

/* Imports */
use super::board::Board;
use uuid;
use rand::{self, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

/* Main */
pub struct Game {
    board: Board,

    /* Player uids, they are options because
        player 2 might not be connected - player
        1 creates game and waits for player 2 */
    white: Option<SocketAddr>,
    black: Option<SocketAddr>,

    /* Game-ID */
    id: String,

    /* Amount of minutes for each player */
    minutes: f64,

    /* Ending time for each player (MS) */
    white_time_remaining: u128,
    black_time_remaining: u128,

    /* Each players last time their clock should be ticking down from (UNIX) */
    white_latest_time: u128,
    black_latest_time: u128,

    /* If each player has made their first move, time will start */
    white_has_moved: bool,
    black_has_moved: bool,
}

/* Method implementations */
impl Game {
    /// Create a new `Game` struct. The `player`
    /// parameter is the uuid of the player who
    /// created the game.
    pub fn new(player: SocketAddr, minutes: f64) -> Self {
        let player_1_white = rand::thread_rng().gen_bool(0.5f64);
        let time_remaining = (1000.0 * 60.0 * minutes) as u128;

        /* Shared fields */
        let shared_fields = Self {
            white: None,
            black: None,

            id: uuid::Uuid::new_v4().as_hyphenated().to_string(),
            board: Board::new(),
            white_time_remaining: time_remaining,
            black_time_remaining: time_remaining,
            black_latest_time: get_unix_time(),
            white_latest_time: get_unix_time(),
            white_has_moved: false,
            black_has_moved: false,
            minutes,
        };

        /* If player 1 should be white */
        if player_1_white {
            Self {
                white: Some(player),
                black: None,

                ..shared_fields
            }
        }else {
            Self {
                white: None,
                black: Some(player),

                ..shared_fields
            }
        }
    }

    /// Append player to empty color, if 
    /// spot is taken, return Err(_)
    pub fn insert_player(&mut self, player: SocketAddr) -> Result<(), ()> {
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
    pub fn white(&self) -> Option<&SocketAddr> { self.white.as_ref() }
    pub fn black(&self) -> Option<&SocketAddr> { self.black.as_ref() }
    pub fn id(&self) -> &String { &self.id }
    pub fn board(&self) -> &Board { &self.board }
    pub fn board_mut(&mut self) -> &mut Board { &mut self.board }

    /* Time */
    pub fn black_time_remaining(&self) -> u128 { self.black_time_remaining }
    pub fn white_time_remaining(&self) -> u128 { self.white_time_remaining }
    pub fn black_time_remaining_mut(&mut self) -> &mut u128 { &mut self.black_time_remaining }
    pub fn white_time_remaining_mut(&mut self) -> &mut u128 { &mut self.white_time_remaining }

    /* Time UNIX */
    pub fn black_latest_time(&self) -> u128 { self.black_latest_time }
    pub fn white_latest_time(&self) -> u128 { self.white_latest_time }
    pub fn black_latest_time_mut(&mut self) -> &mut u128 { &mut self.black_latest_time }
    pub fn white_latest_time_mut(&mut self) -> &mut u128 { &mut self.white_latest_time }

    /* Has moved */
    pub fn black_has_moved(&self) -> bool { self.black_has_moved }
    pub fn white_has_moved(&self) -> bool { self.white_has_moved }
    pub fn white_has_moved_mut(&mut self) -> &mut bool { &mut self.white_has_moved }
    pub fn black_has_moved_mut(&mut self) -> &mut bool { &mut self.black_has_moved }

    /* Minutes */
    pub fn minutes(&self) -> f64 { self.minutes }
    pub fn minutes_mut(&mut self) -> &mut f64 { &mut self.minutes }
}

pub fn get_unix_time() -> u128 {
    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    unix_time.as_millis()
}
