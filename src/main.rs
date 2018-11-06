extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;

#[derive(Template)]
#[template(path = "index.html")]

struct HelloTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "article.amp.html")]
struct BasisTemplate<'a> {
    name: &'a str,
}

use actix_web::{fs, http::Method, server, App, HttpRequest, HttpResponse, Result};
use askama::Template; // bring trait in scope

fn main() {
    let sys = actix::System::new("amp-examples");

    let addr = server::new(|| {
        App::new()
            .resource("/index.html", |r| r.method(Method::GET).f(index))
            .resource("/basis", |r| r.method(Method::GET).f(basis))
            .handler("/static", fs::StaticFiles::new("static").unwrap())
    }).bind("127.0.0.1:8080")
    .unwrap()
    .start();

    let _ = sys.run();
}

fn index(_req: &HttpRequest) -> Result<HttpResponse> {
    let hello = HelloTemplate { name: "world" };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(hello.render().unwrap()))
}

fn basis(_req: &HttpRequest) -> Result<HttpResponse> {
    let basis = BasisTemplate { name: "world" };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(basis.render().unwrap()))
}
