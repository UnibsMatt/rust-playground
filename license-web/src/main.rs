use std::{env, io};
use actix_web::{App, HttpServer, middleware};
mod encrypt;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,actix_server=debug,license_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(encrypt::hallo)
            .service(encrypt::banana)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}