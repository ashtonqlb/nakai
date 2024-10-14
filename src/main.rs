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

#[launch]
fn rocket() -> _ {
    let _ = dotenvy::dotenv(); //TODO: Wrap this in it's own -> Result function rather than have this ugly Thing.

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
