mod core;
mod db;
mod model;
mod route;

#[macro_use]
extern crate rocket;

// #[cfg(test)]
// use std::io;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

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
        .attach(db::init())
        .attach(Template::fairing())
        .mount("/public", FileServer::from("src/view"))
        .mount(
            "/",
            routes![
                route::index,
                route::file::upload,
                route::file::delete,
                route::file::retrieve
            ],
        )
}
