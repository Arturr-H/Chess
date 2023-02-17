/* Imports */
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

/* Main */
pub fn handle_request(
    text: &str,
    data: &Message,
    peers:MutexGuard<
        HashMap<
            SocketAddr,
            UnboundedSender<
                Message
            >
        >
    >
) -> futures_util::future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {

    /* Request data struct */
    #[derive(Deserialize)]
    struct Request {
        board_id: String,
        player_id: String,

        /* Moves */
        from0: String,
        from1: String,
        to0: String,
        to1: String,
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

