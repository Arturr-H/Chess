/* Imports */
use serde_derive::Deserialize;
use chess::game::Game;
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

    /* Create game */
    match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.push(
        Game::new(addr.to_string())
    );

    /* Write the request origin */
    for recp in peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr == &&addr)
        .map(|(_, ws_sink)| ws_sink) {

        recp.unbounded_send(
            Message::Text(
                json!({
                    "status": 200,
                }).to_string()
            )
        ).ok();
    };

    return future::ok(())
}

/* Join game */
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
    addr: SocketAddr
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {

    /* Find game */
    let mut game_found: bool = false;
    for game in match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.iter_mut() {
        if !game.occupied() {
            match game.insert_player(addr.to_string()) {
                Ok(_) => (),
                Err(_) => continue
            };

            game_found = true;
            break;
        }
    };

    /* Game not found */
    if !game_found {
        for recp in peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr == &&addr)
            .map(|(_, ws_sink)| ws_sink) {

            recp.unbounded_send(
                Message::Text(
                    json!({
                        "status": 404,
                    }).to_string()
                )
            ).ok();
        }
    };

    /* Write the request origin */
    for recp in peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr == &&addr)
        .map(|(_, ws_sink)| ws_sink) {

        recp.unbounded_send(
            Message::Text(
                json!({
                    "status": 200,
                }).to_string()
            )
        ).ok();
    };

    return future::ok(())
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
        board_id: String,

        /* Moves */
        from0: String,
        from1: String,
        to0: String,
        to1: String,
    }

    /* Parse request */
    let req = match serde_json::from_str::<Request>(text) {
        Ok(e) => e,
        Err(_) => return future::ok(())
    };

    /* Get moves */
    let parse_error = Message::Text(json!({ "status": 403 }).to_string());
    let (from0, from1, to0, to1) = match ( req.from0.parse::<i8>(), req.from1.parse::<i8>(), req.to0.parse::<i8>(), req.to1.parse::<i8>() ) {
        (Ok(from0), Ok(from1), Ok(to0), Ok(to1)) => {
            (from0, from1, to0, to1)
        },
        _ => return write_origin(peers, addr, parse_error)
    };

    /* Find game and move */
    for game in match games.lock() {
        Ok(e) => e,
        Err(_) => return future::ok(())
    }.iter_mut() {
        if game.id() == &req.board_id {
            match game.board_mut().move_piece_to_coordinate((from0, from1), (to0, to1)) {
                /* Move possible */
                Ok(_) => {
                    return write_origin(peers, addr, Message::Text(
                        json!({
                            "status": 200,
                            "message": "Move successful"
                        }).to_string()
                    ));
                },

                /* Move not possible */
                Err(e) => {
                    return write_origin(peers, addr, Message::Text(
                        json!({
                            "status": 404,
                            "message": e
                        }).to_string()
                    ));
                }
            };
        }
    };

    future::ok(())
}

/* Write to origin */
fn write_origin(peers:MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >,
    addr: SocketAddr,
    message: Message
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {
    for recp in peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr == &&addr)
        .map(|(_, ws_sink)| ws_sink) {

        recp.unbounded_send(
            message.clone()
        ).ok();
    };

    future::ok(())
}
