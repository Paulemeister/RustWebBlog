use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

use actix_files::{Files};

use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/test")]
async fn test() -> impl Responder {

    let mut reg = Handlebars::new();
    reg.register_templates_directory(".hbs","./sites/").unwrap();
    reg.register_templates_directory(".hbs","./blocks/").unwrap();


    let html = match reg.render("index",&json!{""}) {
        Ok(t) => t,
        Err(t) => t.to_string()
    };

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .service(test)
            .route("/hey", web::get().to(manual_hello))
            .service(Files::new("/","./public"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}