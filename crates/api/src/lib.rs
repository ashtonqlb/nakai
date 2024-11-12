use entity::file;
use hostess_service::{
    sea_orm::{Database, DatabaseConnection}, Mutation,
};

use actix_files::Files as Fs;
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result
};

// use entity::files;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;

#[derive(Debug, Clone)]
struct AppState {
    templates: tera::Tera,
    conn: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("uri", request.uri().path());

    let template = &data.templates;
    let body = template
        .render("components/index.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/")]
async fn create(data: web::Data<AppState>, file_form: web::Form<files::Model>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = file_form.into_inner();

    Mutation::create_file(conn, form).await.expect("Could not upload file!");

    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

async fn not_found(data: web::Data<AppState>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("uri", request.uri().path());

    let template = &data.templates;
    let body = template
        .render("components/error/404.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    // load tera templates and build app state
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let state = AppState { templates, conn };

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(Fs::new("/static", "./crates/api/static"))
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .default_service(web::route().to(not_found))
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };
    println!("Starting Hostess server at {server_url}\n");
    server.run().await?;
    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
