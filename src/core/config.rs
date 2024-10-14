// pub mod config {
//     use serde::{Serialize, Deserialize};
//     use rocket::figment::{Figment, providers::{Env, Format, Toml, Serialized}};
   
//     #[derive(Deserialize)]
//     pub struct Config {
//         logging_level: String,
//         port: u16,
//     }

//     impl Default for Config {
//         fn default() -> Config {
//             Config {
//                 logging_level: "verbose".into(),
//                 port: 8000,
//             }
//         }
//     }
// }