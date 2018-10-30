extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama; // for the Template trait and custom derive macro

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative

struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[derive(Template)] // this will generate the code...
#[template(path = "article.amp.html")] // using the template in this path, relative
struct BasisTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

use actix_web::{http::Method, server::HttpServer, App, HttpRequest, HttpResponse, Result};
use askama::Template; // bring trait in scope

fn main() {
    let sys = actix::System::new("guide");

    HttpServer::new(|| {
        vec![
            App::new().resource("/start", |r| r.method(Method::GET).f(start)),
            App::new().resource("/index.html", |r| r.method(Method::GET).f(index)),
        ]
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

fn start(_req: &HttpRequest) -> Result<HttpResponse> {
    let hello = BasisTemplate { name: "world" };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(hello.render().unwrap()))
}
