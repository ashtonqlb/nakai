use rocket_dyn_templates::{context, Template};

pub mod file;

#[get("/")]     
pub async fn index() -> Template {
    Template::render("index", context! { filesize: 0, lifetime: 0 })
}