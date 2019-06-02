use futures::future::Future;
use actix_web::*;
use serde_json::Value as JsonValue;

use crate::tools;

fn hook(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
       .from_err()
       .and_then(|val: JsonValue| {
            if let Some(repo) = val.get("repository") {
                if let Some(name) = repo.get("name") {
                    println!("update repo: {}...", name);
                    tools::deploy(name)?;
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
                resource.post().f(hook);
            })
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run();
}
