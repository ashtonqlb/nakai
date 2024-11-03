/*
  TODO: Rewrite this function to use the DB schema and FormData. 
  When the file is being generated we need to create a database object with it's relevant attributes
  and save it in the /upload folder, along with preserving it's file extension.
  This will change the cURL API but it won't be totally game-breaking.
  It should also allow uploads of multiple files, we just need the formData to allow an array of files.
*/

// #[post("/", data = "<file>")]
// pub async fn upload(file: Data<'_>) -> io::Result<String> {
//     let id = FileID::new();

//     // Process the uploaded file
//     file.open(128.kibibytes()).into_file(id.file_path()).await?;

//     Ok(uri!("http://localhost:8000", retrieve(id)).to_string())
// }


// #[get("/<id>")]
// pub async fn retrieve(id: FileID<'_>) -> Option<RawText<File>> {
//     File::open(id.file_path()).await.map(RawText).ok()
// }

// #[delete("/<id>")]
// pub async fn delete(id: FileID<'_>) -> Option<()> {
//     fs::remove_file(id.file_path()).await.ok()
// }