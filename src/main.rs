mod endpoints;

use actix_web::{middleware, App, HttpServer};

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
