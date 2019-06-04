use futures::future::Future;
use actix_web::{HttpResponse, Error, HttpRequest, server, App, HttpMessage, AsyncResponder};
use serde_json::Value as JsonValue;

use crate::tools;

type AsyncRes = Box<Future<Item = HttpResponse, Error = Error>>;

fn hook(root: &str, req: &HttpRequest) -> AsyncRes {
    req.json()
       .from_err()
       .and_then(|val: JsonValue| {
            if let Some(repo) = val.get("repository") {
                if let Some(name) = repo.get("name") {
                    println!("update repo: {}...", name);
                    tools::deploy(root, name)?;
                    println!("deploy complete");
                }
            }
            Ok(HttpResponse::Ok().into())
       }).responder()
}

pub fn run(root: &str, port: &str) {
    server::new(|| {
        App::new()
            .resource("/test", |resource| {
                resource.get().f(|_| HttpResponse::Ok());
            })
            .resource("/webhook", |resource| {
                resource.post().f(|req| hook(root, req));
            })
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run();
}
