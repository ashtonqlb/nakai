mod core;
mod db;
mod route;

#[macro_use]
extern crate rocket;

use db::Db;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;

fn load_env() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    match load_env() {
        Ok(_) => println!(".env loaded successfully."),
        Err(e) => eprintln!("Failed to load environment variables: {}", e),
    }

    rocket::build()
        .attach(Db::init())
        .attach(Template::fairing())
        .mount("/public", FileServer::from("src/view"))
        .mount(
            "/",
            routes![
                route::index,
                // route::file::upload,
                // route::file::delete,
                // route::file::retrieve
            ],
        )
}