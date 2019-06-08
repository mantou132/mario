use std::collections::HashMap;

use actix_web::{web, App, error, Error, HttpResponse, HttpServer};
use futures::{Future, Stream};
use json::JsonValue;

use crate::tools::deploy;

fn hook(query: web::Query<HashMap<String, String>>, pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    let secret_token = query.get("secret_token").unwrap();
    // future 使用
    let secret_token = secret_token.clone();
    pl.concat2().from_err().and_then(move |body| {
        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let data: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };
        let name = match data["repository"]["name"].as_str() {
            Some(name) => name,
            None => "",
        };
        let clone_url = match data["repository"]["clone_url"].as_str() {
            Some(clone_url) => clone_url,
            None => "",
        };
        if name.is_empty() || clone_url.is_empty() {
            Err(error::ErrorBadRequest(""))
        } else {
            // 一个项目应该对应一个 token
            // 现在暂时使用固定值
            if secret_token != "ABCDEFG" {
                return Err(error::ErrorBadRequest(""));
            }
            // 如何异步执行 deploy？
            deploy(name, clone_url).unwrap();
            Ok(HttpResponse::Ok().body(""))
        }
    })
}

pub fn run(port: &str) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/test").to(HttpResponse::Ok))
            .service(web::resource("/webhook").route(web::post().to_async(hook)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
}
