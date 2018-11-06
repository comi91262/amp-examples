extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    basis_url: &'a str,
}

use actix_web::{fs, http::Method, http::StatusCode, server, App, HttpRequest, HttpResponse, Result};
use askama::Template;

fn basis(req: &HttpRequest) -> Result<HttpResponse> {
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/article.amp.html")))
}


fn index(_req: &HttpRequest) -> Result<HttpResponse> {
    let index = IndexTemplate { basis_url: "http://localhost:8080/basis" };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(index.render().unwrap()))
}

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
