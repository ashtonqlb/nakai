use rocket_dyn_templates::{context, Template};

pub mod file;

// #[derive(Debug, FromForm)]
// #[allow(dead_code)]
// struct File<'v> {
//     file: TempFile<'v>,
//     date: Date,
//     ip: IpAddr
// }

// #[derive(Debug, FromForm)]
// #[allow(dead_code)]
// struct Submit<'v> {
//     file: File<'v>,
// }


#[get("/")]     
pub async fn index() -> Template {
    Template::render("index", context! { filesize: 0, lifetime: 0 })
}