use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

use actix_files::{Files,NamedFile};

use handlebars::Handlebars;
use mariadb::MariaDBPool;
use serde_json::json;

use diesel::prelude::*;

mod mariadb;
mod schema;

use self::schema::blog_entrys::dsl::*;
use self::schema::BlogEntry;

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

#[get("/blog")]
async fn blog(reg: web::Data<Handlebars<'_>>,pool: web::Data<MariaDBPool>,name: web::Path<String>) -> impl Responder {
    
    let mut con = pool.get().expect("Couldn't get Connection");


    let results = blog_entrys
        .limit(5)
        .load::<BlogEntry>(&mut con)
        .expect("Error loading posts");

    println!("{:?}",results);
    
    let html = match reg.render(&name.to_string(),&json!{""}) {
        Ok(t) => t,
        Err(t) => t.to_string()
    };
    
    HttpResponse::Ok().body(html)
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

    let pool1 = mariadb::get_pool();
    println!("Connected to Database");
    let pool = web::Data::new(pool1);

    

    HttpServer::new(move || {
        App::new()
            .app_data(regref.clone())
            .app_data(pool.clone())
            .wrap(Logger::default())
            .service(index)
            .service(test)
            .service(favicon)
            .service(Files::new("/","./public"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}