mod endpoints;
mod frontend;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    color_eyre::install().expect("Misslyckades med att installera color-eyre");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let ip = "127.0.0.5";
    let port = 3000;
    log::info!("Lyssnar på http://{ip}:{port}");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(endpoints::protokoll)
            .service(endpoints::protokoll_lista)
            .service(endpoints::api_protokoll)
            .service(endpoints::api_protokoll_lista)
            .service(endpoints::medlem)
            .service(Files::new("/", "./assets/static").index_file("index.html"))
    })
    .bind((ip, port))?
    .run()
    .await
}
