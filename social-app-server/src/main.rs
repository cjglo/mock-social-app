mod posts;

use actix_web::{web, App, HttpServer};
use std::io;

const WORKER_THREADS: usize = 3;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    HttpServer::new(|| App::new().route("/message", web::post().to(posts::new_message)))
        .workers(WORKER_THREADS)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
