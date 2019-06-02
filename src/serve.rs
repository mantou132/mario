#[macro_use]
use futures::future::Future;
use actix_web::*;

use crate::github::PushEvent;

fn hook(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
       .from_err()
       .and_then(|val: PushEvent| {
           println!("==== BODY ==== {:?}", val);
           Ok(HttpResponse::Ok().into())
       }).responder()
}

pub fn run(_root: &str, port: &str) {
    let addr = format!("127.0.0.1:{}", port);

    server::new(|| {
        App::new()
            .resource("/test", |resource| {
                resource.get().f(|_| HttpResponse::Ok());
            })
            .resource("/webhook", |resource| {
                resource.post().f(hook);
            })
    })
    .bind(&addr)
    .unwrap()
    .run();
}
