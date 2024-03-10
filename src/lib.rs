use actix_web::{http::header::ContentType, HttpResponse, Responder};
use std::fs;

pub fn ul_filer(
    path: &str,
    felmeddelande: &str,
    li_generator: impl Fn(&str, &str) -> String,
) -> impl Responder {
    let mut wip_html = "<ul class=\"lista\">\n".to_string();
    let read_dir_result = match fs::read_dir(&path) {
        Ok(v) => v,
        Err(_e) => {
            dbg!(_e);
            dbg!(path);
            return HttpResponse::NotFound()
                .content_type(ContentType::html())
                .body(format!("<p>{felmeddelande}</p>"));
        }
    };
    for file in read_dir_result {
        let file = match file {
            Ok(v) => v,
            Err(_) => {
                log::warn!("Can't unwrap a DirEntry");
                continue;
            }
        }
        .path();
        let filename = match file.file_name() {
            Some(v) => v,
            None => {
                log::warn!("Can't get a filename");
                continue;
            }
        };
        let filename = match filename.to_str() {
            Some(v) => v,
            None => {
                log::warn!("Can't turn a filename into &str");
                continue;
            }
        };
        let path = match file.parent() {
            Some(v) => v,
            None => {
                log::warn!("Can't get file parent");
                continue;
            }
        };
        let path = match path.to_str() {
            Some(v) => v,
            None => {
                log::warn!("Can't turn a path into &str");
                continue;
            }
        };
        wip_html += &li_generator(filename, path);
    }
    wip_html += "</ul>";
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(wip_html)
}

pub fn mall(titel: &str, content: &str) -> impl Responder {
    let html = include_str!("../assets/mall.html");
    let html = html.replace("{TITEL}", titel);
    let html = html.replace("{CONTENT}", content);
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}
