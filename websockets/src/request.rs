/* Imports */
use chess::game::Game;
use serde_derive::Deserialize;
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::MutexGuard,
};

use tokio_tungstenite::tungstenite::protocol::Message;
use futures_channel::mpsc::UnboundedSender;
use futures_util::future;
use crate::{ChessGames, methods::{create, join, move_, list_games}};

/* Main */
pub fn handle_request(
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
    struct RequestType {
        request_type: String, // "create" | "move" | "join" 
    }

    /* Parse request */
    match serde_json::from_str::<RequestType>(text) {
        Ok(e) => {
            match e.request_type.as_str() {
                "create" => return create(text, peers, games, addr),
                "join" => return join(text, peers, games, addr),
                "move" => return move_(text, peers, games, addr),
                "list_games" => return list_games(peers, games, addr),
                _ => panic!()
            }
        },
        Err(_) => {}
    }

    /* Write to all clients including ourselves */
    for recp in peers.iter().map(|(_, ws_sink)| ws_sink) {
        match recp.unbounded_send(
            Message::Text(
                "Some".into()
            )
        ) {
            Ok(e) => e,
            Err(_) => return future::ok(())
        }
    };

    return future::ok(())
}

