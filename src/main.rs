
#[macro_use] extern crate rocket; 
#[macro_use] extern crate rocket_dyn_templates;
extern crate tera;

use std::fs;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use serde::Deserialize;

#[cfg(test)] mod tests;

#[derive(Debug, Deserialize)]
struct Config {
    file_size: u64,
    file_lifetime: u64,
}

fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let config: Config = toml::de::from_str(&content)?;
    Ok(config)
}

#[get("/")]
fn index() -> Result<Template, String> {
    match read_config("config.toml") {
        Ok(config) => {
            // Destructure the config to extract values
            let Config { file_size, file_lifetime } = config;
            Ok(Template::render("index", context! { filesize: file_size, lifetime: file_lifetime }))
        },
        Err(e) => {
            eprintln!("Error reading config: {}", e);
            Err(format!("Failed to read config: {}", e)) // Return an error message
        }
    }
}

#[launch]
fn rocket() -> _ {    
    //Deserialise values from config.toml and store as values with which to render the home page
    //Compile templates with max_file_size and max_file_lifetime.                  
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
        .attach(Template::fairing())
}