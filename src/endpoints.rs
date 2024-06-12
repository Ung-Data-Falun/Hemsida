use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Path, Redirect},
    Responder, Result,
};
use ung_data_backend::{Medlem, MedlemsLista};

#[get("/api/protokoll/{år}/{månad}/")]
pub async fn api_protokoll_i_månad(argument: Path<(String, String)>) -> impl Responder {
    let path = format!("assets/static/protokoll_raw/{}/{}", argument.0, argument.1);
    ung_data_backend::ul_filer(
        &path,
        "Inga protokoll denna tidsperiod :/",
        |filename: &str, _path: &str| -> String {
            let datum = filename.split(".").next().unwrap_or("");
            let link = datum.replace('-', "/");
            let link = format!("/protokoll/{link}");
            format!("<li><a href=\"{link}\">{datum}</a></li>")
        },
    )
}

#[get("/api/protokoll/{år}/")]
pub async fn api_protokoll_i_år(argument: Path<String>) -> impl Responder {
    let path = format!("assets/static/protokoll_raw/{argument}");
    ung_data_backend::ul_filer(
        &path,
        "Inga protokoll denna tidsperiod :/",
        move |filename: &str, _path: &str| -> String {
            let link = format!("{filename}/");
            format!("<li><a href=\"{link}\">{filename}</a></li>")
        },
    )
}

#[get("/api/protokoll/")]
pub async fn api_protokoll() -> impl Responder {
    let path = format!("assets/static/protokoll_raw");
    ung_data_backend::ul_filer(
        &path,
        "Inga protokoll :/",
        move |filename: &str, _path: &str| -> String {
            let link = format!("{filename}/");
            format!("<li><a href=\"{link}\">{filename}</a></li>")
        },
    )
}

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
    Ok(Redirect::to("/medlem").using_status_code(StatusCode::SEE_OTHER))
}

#[get("/protokoll/{år}/{månad}/")]
pub async fn protokoll_i_månad(argument: Path<(String, String)>) -> impl Responder {
    ung_data_backend::mall(
        &format!("Protokoll {} {}", argument.0, argument.1),
        &format!(
            "<div class=\"replace\" href=\"/api/protokoll/{}/{}/\"></div>",
            argument.0, argument.1
        ),
    )
}

#[get("/protokoll/{år}/")]
pub async fn protokoll_i_år(argument: Path<String>) -> impl Responder {
    ung_data_backend::mall(
        &format!("Protokoll {argument}"),
        &format!("<div class=\"replace\" href=\"/api/protokoll/{argument}/\"></div>",),
    )
}

#[get("/protokoll/{år}/{månad}/{dag}")]
pub async fn protokoll(argument: Path<(String, String, String)>) -> impl Responder {
    ung_data_backend::mall(
        &format!("Protokoll {} {} {}", argument.0, argument.1, argument.2),
        &format!(
            "<div class=\"replace\" href=\"/protokoll_raw/{}/{}/{}-{}-{}.html\"></div>",
            argument.0, argument.1, argument.0, argument.1, argument.2
        ),
    )
}
