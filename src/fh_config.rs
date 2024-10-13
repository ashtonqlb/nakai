pub mod config {
    use std::fs;
    use serde::Deserialize;
    
    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub file_size: u64,
        pub file_lifetime: u64,
    }


    pub fn read_config( file_path: &str, ) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?; //Read file content into a string from the given path
        let config: Config = toml::de::from_str(&content)?; //Unspool that string into two values.
        Ok(config)
    }
}
