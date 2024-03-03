use actix_web::{get, http::header::ContentType, web::Path, HttpResponse, Responder};
use std::{fs, io, path::PathBuf};

#[get("/protokoll/{år}/{månad}/{dag}")]
pub async fn protokoll(argument: Path<(u32, u32, u32)>) -> impl Responder {
    let path = PathBuf::from(format!(
        "assets/protokoll/{:04}/{:02}/{:04} {:02} {:02}.html",
        argument.0, argument.1, argument.0, argument.1, argument.2
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

/*
#[get("/protokoll/{år}/{månad}")]
pub async fn protokoll_i_månad(argument: Path<(u32, u32)>) -> impl Responder {

}
*/
