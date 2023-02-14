/* Imports */
use actix_web::{ get, Responder, HttpResponse, web };

/* Constants */
const DIST_PATH: &'static str = "./frontend/dist/";
const HTMX_PATH: &'static str = "./frontend/bin/htmx.min.js";
const INDEX_PATH: &'static str = "./frontend/src/index.html";

/* Routes */
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

/* Index html file */
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read(INDEX_PATH).unwrap_or(Vec::new()))
}
