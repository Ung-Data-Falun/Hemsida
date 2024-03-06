mod endpoints;
mod frontend;

use actix_web::{middleware, App, HttpServer};
use actix_files::Files;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    color_eyre::install().expect("Misslyckades med att installera color-eyre");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Lyssnar på http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(endpoints::protokoll)
            .service(Files::new("/", "./assets/static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
