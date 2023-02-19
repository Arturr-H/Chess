/* Imports */
use serde_derive::Deserialize;
use chess::{game::Game, piece::Color};
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
pub fn create(
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

    /* Create game */
    let game = Game::new(addr);
    let is_white = game.white().is_some();

    match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.push(game);

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
pub fn join(
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

    /* Find game */
    for game in match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.iter_mut() {
        if !game.occupied() {
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
                    "peer_addr": addr.to_string()
                }).to_string()
            );
            
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

/* Write to origin */
fn write_origin(
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
