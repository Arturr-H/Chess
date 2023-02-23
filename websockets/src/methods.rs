/* Imports */
use serde_derive::{Deserialize, Serialize};
use chess::{game::{Game, self}, piece::Color, board::Board};
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::MutexGuard,
};

use tokio_tungstenite::tungstenite::protocol::Message;
use futures_channel::mpsc::UnboundedSender;
use futures_util::future;
use crate::ChessGames;

/* Create game */
#[allow(unused_must_use)]
pub fn create(
    text: &str,
    peers:MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >,
    games: ChessGames,
    addr: SocketAddr
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {
    #[derive(Deserialize)]
    struct Request {
        minutes: f64,
    }

    /* Parse request */
    let req = match serde_json::from_str::<Request>(text) {
        Ok(e) => e,
        Err(_) => return write_origin(&peers, &[addr], &Message::Text(json!({ "status": 403, "message": "Couldn't parse request" }).to_string()))
    };

    /* Create game */
    let game = Game::new(addr, req.minutes);
    let is_white = game.white().is_some();

    match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.push(game);

    let found_games = match convert_to_games_response(&games) {
        Some(e) => e,
        None => return future::ok(())
    };
    write_all_origins(&peers, &Message::Text(json!({
        "status": 200,
        "type": "update_games_listing",
        "games": found_games
    }).to_string()));

    /* Write the request origin */
    for recp in peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr == &&addr)
        .map(|(_, ws_sink)| ws_sink) {

        recp.unbounded_send(
            Message::Text(
                json!({
                    "status": 200,
                    "message": "Game has been created",
                    "type": "create",
                    "is_white": is_white,
                    "peer_addr": addr.to_string()
                }).to_string()
            )
        ).ok();
    };

    return future::ok(())
}

/* Join game */
#[allow(unused_must_use)]
pub fn join(
    text: &str,
    peers:MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >,
    games: ChessGames,
    addr: SocketAddr,
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {

    /* Respond with this struct */
    #[derive(Deserialize)]
    struct Request {
        game_id: String,
    }

    /* Parse request */
    let req = match serde_json::from_str::<Request>(text) {
        Ok(e) => e,
        Err(_) => return write_origin(&peers, &[addr], &Message::Text(json!({ "status": 403, "message": "Couldn't parse request" }).to_string()))
    };

    let found_games = match convert_to_games_response(&games) {
        Some(e) => e,
        None => return future::ok(())
    };

    /* Find game */
    for game in match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.iter_mut() {
        if !game.occupied() && game.id() == &req.game_id {
            match game.insert_player(addr) {
                Ok(_) => (),
                Err(_) => continue
            };

            /* Send to all players in game */
            let msg = Message::Text(
                json!({
                    "status": 200,
                    "message": "Game started!",
                    "game_id": game.id(),
                    "type": "start",
                    "is_white": game.white().unwrap() == &addr,
                    "peer_addr": addr.to_string(),
                    "white_time_left": game.white_time_remaining(),
                    "black_time_left": game.black_time_remaining()
                }).to_string()
            );
            
            write_all_origins(&peers, &Message::Text(json!({
                "status": 200,
                "type": "update_games_listing",
                "games": found_games.iter().filter(|e| &e.id != game.id()).collect::<Vec<&GameInfo>>()
            }).to_string()));

            /* Write the request origin */
            return write_origin(&peers, &[
                *game.black().unwrap(),
                *game.white().unwrap()
            ], &msg)
        }
    };

    /* Game not found */
    return write_origin(&peers, &[addr], 
        &Message::Text(
            json!({
                "status": 404,
                "message": "Game not found!",
                "type": "game_not_found"
            }).to_string()
        )
    )
}

/* Make move */
pub fn move_(
    text: &str,
    peers:MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >,
    games: ChessGames,
    addr: SocketAddr
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {

    /* Request data struct */
    #[derive(Deserialize)]
    struct Request {
        game_id: String,

        /* Moves */
        from0: String,
        from1: String,
        to0: String,
        to1: String,
    }

    /* Parse request */
    let req = match serde_json::from_str::<Request>(text) {
        Ok(e) => e,
        Err(_) => return write_origin(&peers, &[addr], &Message::Text(json!({ "status": 403, "message": "Couldn't parse request" }).to_string()))
    };

    /* Get moves */
    let parse_error = Message::Text(json!({ "status": 403, "message": "Parsing error" }).to_string());
    let (from0, from1, to0, to1) = match ( req.from0.parse::<i8>(), req.from1.parse::<i8>(), req.to0.parse::<i8>(), req.to1.parse::<i8>() ) {
        (Ok(from0), Ok(from1), Ok(to0), Ok(to1)) => {
            (from0, from1, to0, to1)
        },
        _ => return write_origin(&peers, &[addr], &parse_error)
    };

    /* Find game and move */
    for game in match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.iter_mut() {
        if game.id() == &req.game_id {

            /* Get the color of the `peer_addr` user */
            let addr_color = if let Some(w) = game.white() {
                if w == &addr { Color::White }
                else { Color::Black }
            }else if let Some(b) = game.black() {
                if b == &addr { Color::Black }
                else { Color::White }
            }else {
                Color::White
            };

            /* Check if it's the right players turn */
            if game.board().turn() != addr_color {
                return write_origin(&peers, &[addr], &Message::Text(
                    json!({
                        "status": 404,
                        "message": "Not your turn",
                        "type": "error"
                    }).to_string()
                ));
            };

            match game.board_mut().move_piece_to_coordinate((from0, from1), (to0, to1)) {
                /* Move possible */
                Ok(_) => {
                    /* Switch player */
                    game.board_mut().toggle_turn();

                    /* If we should start the timer or not for both players */
                    let start_timer: bool = game.white_has_moved() && game.black_has_moved();

                    /* Subtract time */
                    match game.board().turn() {
                        Color::Black => {
                            if start_timer {
                                *game.white_time_remaining_mut() -= game::get_unix_time() - game.white_latest_time();
                                *game.black_latest_time_mut() = game::get_unix_time();
                            }else {
                                *game.black_latest_time_mut() = game::get_unix_time();
                                *game.white_latest_time_mut() = game::get_unix_time();
                            }
                                
                            *game.white_has_moved_mut() = true;
                        },
                        Color::White => {
                            if start_timer {
                                *game.black_time_remaining_mut() -= game::get_unix_time() - game.black_latest_time();
                                *game.white_latest_time_mut() = game::get_unix_time();
                            }else {
                                *game.black_latest_time_mut() = game::get_unix_time();
                                *game.white_latest_time_mut() = game::get_unix_time();
                            }

                            *game.black_has_moved_mut() = true;
                        },
                    };

                    /* Look if is in checkmate */
                    if game.board().is_checkmated(game.board().turn()) {
                        return write_origin(
                            &peers,
                            &[*game.black().unwrap(), *game.white().unwrap()],
                            &Message::Text(json!({
                                "status": 200,
                                "type": "win",
                                "board": game.board(),
                                "lost": game.board().turn(),
                                "from0": from0,
                                "from1": from1,
                                "to0": to0,
                                "to1": to1,

                                "turn": game.board().turn(),

                                "time_left_black": game.black_time_remaining(),
                                "time_left_white": game.white_time_remaining(),
                            }).to_string())
                        )
                    }

                    /* Look if is in stalemated */
                    if game.board().is_stalemated() {
                        return write_origin(
                            &peers,
                            &[*game.black().unwrap(), *game.white().unwrap()],
                            &Message::Text(json!({
                                "status": 200,
                                "type": "stalemate",

                                "board": game.board(),
                                "lost": game.board().turn(),
                                "from0": from0,
                                "from1": from1,
                                "to0": to0,
                                "to1": to1,
                            }).to_string())
                        )
                    }

                    return write_origin(&peers, &[*game.black().unwrap(), *game.white().unwrap()], &Message::Text(
                        json!({
                            "status": 200,
                            "message": "Move successful",
                            "board": game.board(),
                            "type": "move",
                            "from0": from0,
                            "from1": from1,
                            "to0": to0,
                            "to1": to1,

                            "turn": game.board().turn(),

                            "time_left_black": game.black_time_remaining(),
                            "time_left_white": game.white_time_remaining(),
                        }).to_string()
                    ));
                },

                /* Move not possible */
                Err(e) => {
                    /* Only write to self origin because other
                        player won't need to hear about the current
                        players moves being rejected */
                    return write_origin(&peers, &[addr], &Message::Text(
                        json!({
                            "status": 404,
                            "message": e,
                            "type": "error"
                        }).to_string()
                    ));
                }
            };
        }
    };

    future::ok(())
}

/* List available games */
pub fn list_games(
    peers:MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >,
    games: ChessGames,
    addr: SocketAddr
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {
    let found_games = match convert_to_games_response(&games) {
        Some(e) => e,
        None => return future::ok(())
    };

    return write_origin(&peers, &[addr], 
        &Message::Text(
            json!({
                "status": 200,
                "type": "games_listing",
                "games": found_games
            }).to_string()
        )
    )
}

/* Write to origin */
pub fn write_origin(
    peers:&MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >,
    addrs: &[SocketAddr],
    message: &Message
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {
    for recp in peers
        .iter()
        .filter(|(peer_addr, _)| addrs.contains(&peer_addr))
        .map(|(_, ws_sink)| ws_sink) {

        recp.unbounded_send(
            message.clone()
        ).ok();
    };

    future::ok(())
}

/* Write to all origins */
pub fn write_all_origins(
    peers:&MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >,
    message: &Message
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {
    for recp in peers
        .iter()
        .map(|(_, ws_sink)| ws_sink) {
        recp.unbounded_send(
            message.clone()
        ).ok();
    };

    future::ok(())
}

/* Respond with this struct */
#[derive(Serialize)]
pub struct GameInfo {
    id: String,
    creator: String,
    minutes: f64
}

/* Convert games into response */
pub fn convert_to_games_response(games: &ChessGames) -> Option<Vec<GameInfo>> {

    let mut found_games: Vec<GameInfo> = Vec::new();

    /* Find game */
    for game in match games.lock() {
        Ok(e) => e,
        Err(_) => return None
    }.iter() {
        if !game.occupied() {
            found_games.push(GameInfo {
                id: game.id().to_string(),
                minutes: game.minutes(),
                creator: String::from("Unknown creator")
            });
        };
    };

    Some(found_games)
}