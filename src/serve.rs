use actix_web::{web, App, HttpServer, HttpResponse, Error};
use futures::{Future, Stream};
use json::JsonValue;

use crate::tools::deploy;

fn hook(pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> + 'static {
    pl.concat2().from_err().and_then(|body| {
        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let data: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };
        let res = match data["repository"]["name"].as_str() {
            Some(name) => {
                println!("{:?}", name);
                deploy(name).unwrap();
                String::new()
            }
            None => (json::object! {"err" => "bad request!" }).dump()
        };

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(res))
    })
}

pub fn run(port: &str) -> std::io::Result<()> {
    HttpServer::new(
        || {
            App::new()
            .service(web::resource("/test").to(HttpResponse::Ok))
            .service(web::resource("/webhook").route(web::post().to_async(hook)))
        })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
}
