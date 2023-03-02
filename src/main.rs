use actix_web::{get,  web, App, HttpRequest, HttpResponse , HttpServer, Responder, routes};
use actix_web::middleware::Logger;

use actix_files::{Files,NamedFile};

use diesel::r2d2::{Pool,ConnectionManager};
use diesel::prelude::*;

use handlebars::Handlebars;
use serde_json::json;

mod mariadb;
mod schema;

use self::schema::BlogEntries::dsl::*;
use self::schema::BlogEntry;

#[routes]
#[get("/favicon")]
#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open("public/favicon.ico")
}

#[get("/")]
async fn index<'a>(appdata: web::Data<AppData<'a>>) -> impl Responder {

    let html = match appdata.registry.render("index",&json!{""}) {
        Ok(t) => t,
        Err(t) => t.to_string()
    };

    HttpResponse::Ok().body(html)
}

#[get("/blog")]
async fn blog<'a>(appdata: web::Data<AppData<'a>>) -> impl Responder {
    
    let mut con = appdata.pool.get().expect("Couldn't get Connection");

    let results = BlogEntries
        .limit(5)
        .load::<BlogEntry>(&mut con)
        .expect("Error loading posts");

    let json = json!({"data": results});
    
    println!("{}",json);

    let html = match appdata.registry.render("blog",&json) {
        Ok(t) => t,
        Err(t) => t.to_string()
    };
    
    HttpResponse::Ok().body(html)
}

#[routes]
#[get("{name}")]
#[delete("{name}")]
#[get("/index")]
#[get("/info")]
#[get("/new")]

async fn test<'a>(req: HttpRequest, appdata: web::Data<AppData<'a>>) -> impl Responder {
    let name: String = req.match_info().query("name").parse().unwrap();
    println!("auto: {}",&name.to_string());
    let html = match appdata.registry.render(&name.to_string(),&json!{""}) {
        Ok(t) => t,
        Err(t) => t.to_string()
    };

    HttpResponse::Ok().body(html)
}

#[derive(Clone)]
struct AppData<'a> {
    registry: handlebars::Handlebars<'a>,
    pool: Pool<ConnectionManager<MysqlConnection>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut reg = Handlebars::new();
    reg.register_templates_directory(".hbs","./sites/").unwrap();
    reg.register_templates_directory(".hbs","./blocks/").unwrap();
    
    let pool = mariadb::get_pool();
    println!("Connected to Database");

    let appdata = web::Data::new(AppData{
        registry: reg,
        pool: pool
    });

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::new("%U->%{handler}xi")
            .custom_request_replace("handler", |req| req.match_name().unwrap_or("Not Found").to_string()))
        .app_data(web::Data::clone(&appdata))
        .service(index)
        .service(favicon)
        .service(blog)
        .service(test)
        .service(Files::new("/","./public"))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}