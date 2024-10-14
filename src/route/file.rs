use std::io;
use rocket::data::{Data, ToByteUnit};
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};
use crate::core::uuid::FileID;


#[post("/", data = "<file>")]
pub async fn upload(file: Data<'_>) -> io::Result<String> {
    let id = FileID::new();
    file
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;
    
    Ok(uri!("http://localhost:8000", retrieve(id)).to_string()) //TODO: Read from .env
}

#[get("/<id>")]
pub async fn retrieve(id: FileID<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}

#[delete("/<id>")]
pub async fn delete(id: FileID<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}