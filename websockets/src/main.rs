#![allow(
    unused_imports,
    dead_code
)]

/* Modules */
mod request;
mod methods;

/* Imports */
use chess::game::Game;
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

use crate::methods::{convert_to_games_response, write_origin, write_all_origins};

/* Constants */
const ADDRESS: &'static str = "127.0.0.1";
const PORT: u16 = 8081;

/* Types */
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type ChessGames = Arc<Mutex<Vec<Game>>>;

#[tokio::main]
async fn main() {
    /* Initialize stream */
    let addr = format!("{ADDRESS}:{PORT}");
    let server = TcpListener::bind(&addr).await.unwrap();
    println!("{addr}");

    /* Create peer map and games */
    let peers:PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let games:ChessGames = Arc::new(Mutex::new(Vec::new()));

    /* Incoming requests */
    while let Ok((stream, addr)) = server.accept().await {
        tokio::spawn(handle_connection(peers.clone(), games.clone(), stream, addr));
    }
}

#[allow(unused_must_use)]
async fn handle_connection(peer_map: PeerMap, games: ChessGames, raw_stream: TcpStream, addr: SocketAddr) {
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
                return request::handle_request(text, peers, games.clone(), addr)
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
    match peer_map.lock() {
        Ok(e) => e,
        Err(_) => return
    }.remove(&addr);

    /* Update games listing */
    let player_left_message = &Message::Text(json!({
        "type": "player_leave"
    }).to_string());

    let peers = match peer_map.lock() {
        Ok(e) => e,
        Err(_) => return
    };

    match games.lock() {
        Ok(mut e) => {
            for index in 0..e.len() {
                if let Some(black) = e[index].black() {
                    if black == &addr {
                        if let Some(white) = e[index].white() {

                            /* Send to white */
                            write_origin(&peers, &[*white], player_left_message);
                        }

                        /* Remove game */
                        e.remove(index);
                    }
                }else if let Some(white) = e[index].white() {
                    if white == &addr {
                        if let Some(black) = e[index].black() {
                            /* Send to black */
                            write_origin(&peers, &[*black], player_left_message);
                        }

                        /* Remove game */
                        e.remove(index);
                    }
                }
            }
        },
        Err(_) => return
    };

    let games = convert_to_games_response(&games);
    write_all_origins(&peers, &Message::Text(json!({
        "status": 200,
        "type": "update_games_listing",
        "games": games
    }).to_string()));
}
