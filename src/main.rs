use actix_web::{
    get, http::header::ContentType, middleware, web, App, HttpResponse, HttpServer, Responder,
};
use std::{fs, io, path};

#[get("/protokoll/{ar}/{manad}/{dag}")]
async fn protokoll(argument: web::Path<(u32, u32, u32)>) -> impl Responder {
    let path = path::PathBuf::from(format!(
        "assets/protokoll/{:04} {:02} {:02}.html",
        argument.0, argument.1, argument.2
    ));
    match fs::read(path) {
        Ok(value) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(value),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("Finns inget protokoll från den dagen :/"),
            _ => HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body("Servern funkar inte :/"),
        },
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    color_eyre::install().expect("Misslyckades med att installera color-eyre");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Lyssnar på http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(protokoll)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
