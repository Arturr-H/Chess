/* Imports */
use std::sync::Mutex;
use actix_web::{ get, Responder, HttpResponse, web::{self, Data}, HttpRequest, post };
use crate::board::Board;
use serde_json::json;

/* Constants */
const DIST_PATH: &'static str = "./frontend/dist/";
const HTMX_PATH: &'static str = "./frontend/bin/htmx.min.js";
const INDEX_PATH: &'static str = "./frontend/src/index.html";

/* STATIC ROUTES */
#[get("/dist/{name}")]
async fn js_dist(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().content_type("text/javascript").body(
        match std::fs::read(DIST_PATH.to_string() + name.as_str()) {
            Ok(e) => e,
            Err(_) => Vec::new()
        }
    )
}
#[get("/htmx.min.js")]
async fn js_htmx() -> impl Responder {
    HttpResponse::Ok().content_type("text/javascript").body(
        match std::fs::read(HTMX_PATH) {
            Ok(e) => e,
            Err(_) => Vec::new()
        }
    )
}
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read(INDEX_PATH).unwrap_or(Vec::new()))
}

/* Functionality routes */
#[post("/move")]
async fn move_(req: HttpRequest, data: Data<Mutex<Board>>) -> impl Responder {
    let headers = req.headers();

    /* Get moves */
    let (from0, from1, to0, to1) = match (
        headers.get("from0"),
        headers.get("from1"),
        headers.get("to0"),
        headers.get("to1")
    ) {
        (Some(from0), Some(from1), Some(to0), Some(to1)) => {
            match ( from0.to_str(), from1.to_str(), to0.to_str(), to1.to_str() ) {

                (Ok(from0), Ok(from1), Ok(to0), Ok(to1)) => {
                    match ( from0.parse::<i8>(), from1.parse::<i8>(), to0.parse::<i8>(), to1.parse::<i8>() ) {

                        (Ok(from0), Ok(from1), Ok(to0), Ok(to1)) => {
                            (from0, from1, to0, to1)
                        },
                        _ => panic!()
                    }
                },
                _ => panic!()
            }
        },
        _ => panic!()
    };

    let board = &mut data.lock().unwrap();
    match board.move_piece_to_coordinate((from0, from1), (to0, to1)) {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "status": 200,
                "message": "Success",
                "board": &**board
            }))  
        },
        Err(msg) => {
            HttpResponse::Ok().json(json!({
                "status": 4002,
                "message": msg
            }))
        }
    }
}
