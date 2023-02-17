/* Modules */
mod request;

/* Imports */
use serde_derive::Deserialize;
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{ Arc, Mutex },
};

use futures_channel::mpsc::{ unbounded, UnboundedSender };
use futures_util::{ future, pin_mut, stream::TryStreamExt, StreamExt };

use tokio::net::{ TcpListener, TcpStream };
use tokio_tungstenite::tungstenite::protocol::Message;

/* Constants */
const ADDRESS: &'static str = "127.0.0.1";
const PORT: u16 = 8081;

/* Types */
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    /* Initialize stream */
    let addr = format!("{ADDRESS}:{PORT}");
    let server = TcpListener::bind(&addr).await.unwrap();
    println!("{addr}");

    /* Create peer map */
    let peers:PeerMap = Arc::new(Mutex::new(HashMap::new()));

    /* Incoming requests */
    while let Ok((stream, addr)) = server.accept().await {
        tokio::spawn(handle_connection(peers.clone(), stream, addr));
    }
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    let ws_stream = match tokio_tungstenite::accept_async(raw_stream).await {
        Ok(e) => e,
        Err(_) => return
    };

    /* Insert to peers */
    let (tx, rx) = unbounded();
    match peer_map.lock() { Ok(e) => e, Err(_) => return }.insert(addr, tx);
    let (outgoing, incoming) = ws_stream.split();

    /* Message loop */
    let broadcast_incoming = incoming.try_for_each(|data| {
        let peers = match peer_map.lock() {
            Ok(e) => e,
            Err(_) => return future::ok(())
        };

        match &data {
            Message::Text(text) => {
                return request::handle_request(text, &data, peers)
            },
            _ => return future::ok(())
        };
    });


    /* Enable user to also recieve messages */
    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);

    /* Run recv / broadcast */
    future::select(broadcast_incoming, receive_from_others).await;

    /* Remove peer */
    peer_map.lock().unwrap().remove(&addr);
}
