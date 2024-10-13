#[macro_use]
extern crate rocket;
mod uuid;
#[cfg(test)]
mod tests;
use std::io;

// use fh_config::config::{read_config, Config};
use rocket::data::{Data, ToByteUnit};
use rocket::fs::FileServer;
use rocket::http::uri::Absolute;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};

use uuid::FileID;
use rocket_dyn_templates::{context, Template};

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[post("/", data = "<file>")]
async fn upload(file: Data<'_>) -> io::Result<String> {
    let id = FileID::new();
    file
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[get("/<id>")]
async fn retrieve(id: FileID<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}

#[delete("/<id>")]
async fn delete(id: FileID<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { filesize: 64, lifetime: 128 })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload, delete, retrieve])
        .mount("/public", FileServer::from("public"))
        .attach(Template::fairing())
}
