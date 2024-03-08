use actix_web::{get, web::Path, Responder};

#[get("/api/protokoll/{år}/{månad}/")]
pub async fn api_protokoll_i_månad(argument: Path<(String, String)>) -> impl Responder {
    let path = format!("assets/static/protokoll_raw/{}/{}", argument.0, argument.1);
    ung_data_backend::ul_filer(&path, "Inga protokoll denna tidsperiod :/", |filename: &str, _path: &str| -> String {
        let datum = filename.split(".").next().unwrap_or("");
        let link = datum.replace('-', "/");
        let link = format!("/protokoll/{link}");
        format!("<li><a href=\"{link}\">{datum}</a></li>")
    })
}

#[get("/api/protokoll/{år}/")]
pub async fn api_protokoll_i_år(argument: Path<String>) -> impl Responder {
    let path = format!("assets/static/protokoll_raw/{argument}");
    ung_data_backend::ul_filer(&path, "Inga protokoll denna tidsperiod :/", move |filename: &str, _path: &str| -> String {
        let link = format!("{filename}/");
        format!("<li><a href=\"{link}\">{filename}</a></li>")
    })
}

#[get("/api/protokoll/")]
pub async fn api_protokoll() -> impl Responder {
    let path = format!("assets/static/protokoll_raw");
    ung_data_backend::ul_filer(&path, "Inga protokoll :/", move |filename: &str, _path: &str| -> String {
        let link = format!("{filename}/");
        format!("<li><a href=\"{link}\">{filename}</a></li>")
    })
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
        &format!(
            "<div class=\"replace\" href=\"/api/protokoll/{argument}/\"></div>",
        ),
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
