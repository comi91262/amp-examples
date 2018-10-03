extern crate actix;
extern crate actix_web;

use actix_web::{http::Method, server::HttpServer, App, HttpRequest};

fn main() {
    let sys = actix::System::new("guide");

    HttpServer::new(|| {
        vec![
            App::new().resource("/index.html", |r| r.method(Method::GET).f(index)),
            App::new().resource("/start", |r| r.method(Method::GET).f(start)),
        ]
    }).bind("127.0.0.1:8080")
    .unwrap()
    .start();

    let _ = sys.run();
}

fn index(_req: &HttpRequest) -> &'static str {
    r"
    こちらはAMPのデモになります
    - AMPの基本 (http://127.0.0.1:8080/first)
    "
}

fn start(_req: &HttpRequest) -> &'static str {
    r"
    こちらはAMPのデモになります
    - AMPの基本 (http://127.0.0.1:8080/first)
    "
}
