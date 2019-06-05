use actix_web::{web, App, HttpServer, HttpResponse, Error};
use futures::{Future, Stream};
use json::JsonValue;

use crate::tools::deploy;

fn hook(root: &'static str, pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> + 'static {
    pl.concat2().from_err().and_then(move |body| {
        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let data: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };
        let res = match data["repository"]["name"].as_str() {
            Some(name) => {
                println!("{:?}", name);
                deploy(root, name);
                String::new()
            }
            None => (json::object! {"err" => "bad request!" }).dump()
        };

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(res))
    })
}

pub fn run(root: &'static str, port: &str) -> std::io::Result<()> {
    let root = root.clone();
    HttpServer::new(
        move || {
            App::new()
            .service(web::resource("/test").to(HttpResponse::Ok))
            .service(web::resource("/webhook").route(web::post().to_async(move |pl| hook(root, pl))))
        })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
}
