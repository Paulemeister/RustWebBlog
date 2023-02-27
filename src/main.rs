use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

use actix_files::{Files,NamedFile};

use handlebars::Handlebars;
use serde_json::json;


#[get("/favicon")]
async fn favicon() -> impl Responder {
    NamedFile::open("public/favicon.ico")
}

#[get("/")]
async fn index(reg: web::Data<Handlebars<'_>>) -> impl Responder {

    let html = match reg.render("index",&json!{""}) {
        Ok(t) => t,
        Err(t) => t.to_string()
    };

    HttpResponse::Ok().body(html)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/{test}")]
async fn test(reg: web::Data<Handlebars<'_>>,name: web::Path<String>) -> impl Responder {

    let html = match reg.render(&name.to_string(),&json!{""}) {
        Ok(t) => t,
        Err(t) => t.to_string()
    };

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let mut reg = Handlebars::new();
    reg.register_templates_directory(".hbs","./sites/").unwrap();
    reg.register_templates_directory(".hbs","./blocks/").unwrap();

    let regref = web::Data::new(reg);

    HttpServer::new(move || {
        App::new()
            .app_data(regref.clone())
            .wrap(Logger::default())
            .service(index)
            .service(echo)
            .service(test)
            .service(favicon)
            .route("/hey", web::get().to(manual_hello))
            .service(Files::new("/","./public"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}