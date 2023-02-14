/* Global allowances */
#![allow(
    dead_code
)]

/* Imports */
use actix_web::{ App, HttpServer };
use actix_files;

/* Modules */
mod routes;
mod board;
mod traits;

/* Main */
#[actix_web::main]
async fn main() -> std::io::Result<()> {

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
