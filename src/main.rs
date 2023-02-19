/* Global allowances */
#![allow(
    dead_code,
    unused_imports
)]

/* Modules */
pub mod routes;
pub mod board;
pub mod traits;
pub mod piece;
pub mod game;

#[path = "./chess-pieces/bundle.rs"]
pub mod bundle;

/* Imports */
use std::{io::{Read, stdin, stdout, Write}, sync::Mutex};
use actix_web::{ App, HttpServer, web::Data };
use actix_files;
use chess::{bundle::{rook::Rook, pawn::Pawn, bishop::Bishop, knight::Knight, king::King, queen::Queen}, traits::PieceMethods, board::{Board, Tile}};

/* Main */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let appdata = Data::new(Mutex::new(board::Board::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&appdata))

            /* Static files */
            .service(actix_files::Files::new("/static", "./frontend/src/").index_file(""))
            .service(actix_files::Files::new("/pieces", "./frontend/pieces/").index_file(""))
            .service(routes::index)
            .service(routes::js_dist)
            .service(routes::js_htmx)
    })
    .workers(12)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
