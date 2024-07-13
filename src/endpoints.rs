use actix_web::{
    get, http::StatusCode, post, web, HttpResponse, Responder, Result
};
use ung_data_backend::{mall, ul_filer, Medlem, MedlemsLista};

#[post("/api/medlem/")]
pub async fn medlem(medlem: web::Form<Medlem>) -> Result<impl Responder> {
    println!("test");
    let mut current_members: MedlemsLista = toml::from_str(
        &tokio::fs::read_to_string("medlemmar.toml")
            .await
            .unwrap_or("medlemmar = []".into()),
    )
    .unwrap();
    current_members.medlemmar.push(medlem.0);
    tokio::fs::write(
        "medlemmar.toml",
        toml::to_string_pretty(&current_members).unwrap(),
    )
    .await?;
    Ok(web::Redirect::to("/medlem").using_status_code(StatusCode::SEE_OTHER))
}

#[get("/api/protokoll/")]
pub async fn api_protokoll_lista() -> impl Responder {
    ul_filer(
        "assets/static/markdown/protokoll/",
        "Inga protokoll :/",
        |filename: &str, _path: &str| -> String {
            format!(r#"<li><a href="{filename}">{filename}</a></li>"#)
        },
    )
}

#[get("/api/protokoll/{file}")]
pub async fn api_protokoll(file: web::Path<String>) -> impl Responder {
    let contents = match std::fs::read_to_string(format!("assets/static/markdown/protokoll/{file}")) {
        Ok(v) => v,
        Err(_e) => { dbg!(_e); return HttpResponse::NotFound().body(":(")},
    };

    let contents = markdown::to_html(&contents);

    HttpResponse::Ok().body(contents)
}

#[get("/protokoll/")]
pub async fn protokoll_lista() -> impl Responder {
    mall("Protokoll", r#"<div class="replace" href="/api/protokoll/"></div>"#)
}


#[get("/protokoll/{file}")]
pub async fn protokoll(argument: web::Path<String>) -> impl Responder {
    mall(
        &argument,
        &format!(
            r#"<div class="replace" href="/api/protokoll/{argument}"></div>"#,
        ),
    )
}
