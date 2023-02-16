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

#[path = "./chess-pieces/bundle.rs"]
pub mod bundle;

/* Imports */
use actix_web::{ App, HttpServer };
use actix_files;
use chess::{bundle::{rook::Rook, pawn::Pawn, bishop::Bishop, knight::Knight, king::King, queen::Queen}, traits::PieceMethods, board::{Board, Tile}};

/* Main */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let b = chess::piece::Color::Black;
    let w = chess::piece::Color::White;
    let non = Tile::Empty;
    let piec = Tile::Piece;

    let mut board = Board::from(
        vec![
            vec![ non, non, non, non, piec(King::new(b)), piec(Bishop::new(b)), piec(Knight::new(b)), piec(Rook::new(b)) ],
            // vec![ piec(Pawn::new(b)); 8 ],
            vec![ non; 8 ],
            vec![ non; 8 ],
            vec![ non; 8 ],
            vec![ non; 8 ],
            vec![ non; 8 ],
            vec![ non; 8 ],
            // vec![ piec(Pawn::new(w)); 8 ],
            vec![ piec(Rook::new(w)), piec(Knight::new(w)), piec(Bishop::new(w)), piec(Queen::new(w)), piec(King::new(w)), piec(Bishop::new(w)), piec(Knight::new(w)), piec(Rook::new(w)) ],
        ]
    );
    println!("{:?}", board);
    println!("is white check: {:?}", board.is_in_check(chess::piece::Color::White));
    println!("is black check: {:?}", board.is_in_check(chess::piece::Color::Black));
    board.move_piece_to_coordinate((3, 7), (0, 4)).unwrap();
    println!("{:?}", board);
    println!("is white check: {:?}", board.is_in_check(chess::piece::Color::White));
    println!("is black check: {:?}", board.is_in_check(chess::piece::Color::Black));

    // let appdata = Data::new(Mutex::new(appdata::AppData::from_file()));
    HttpServer::new(move || {
        App::new()
            /* Set maximum payload size to 32MiB */
            // .app_data(web::PayloadConfig::new(1 << 25))
            // .app_data(Data::clone(&appdata))

            /* Static files */
            .service(routes::index)
            .service(actix_files::Files::new("/static", "./frontend/src/").index_file(""))

            /* Static files */
            .service(routes::js_dist)
            .service(routes::js_htmx)
    })
    .workers(12)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
