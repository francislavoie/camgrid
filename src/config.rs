extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    paths: Option<Vec<String>>,
}

impl Config {
    /// Creates a new empty config
    fn new() -> Config {
        Config {
            paths: Some(vec![]),
        }
    }

    /// Loads the config from file at given path, or creates a default config
    pub fn load(path: &str) -> Config {
        // Load the file, or create a default Config if no file
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(..) => return Config::new(),
        };

        // If we loaded a file, read the file contents
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Deserialize the file as TOML into our Config struct
        let mut config: Config = toml::from_str(contents.as_ref()).unwrap();

        // Ensure we have an empty vec if loaded file had none
        if config.paths.is_none() {
            config.paths.replace(vec![]);
        }

        config
    }

    /// Serializes the config and stores it to file at given path
    pub fn save(&self, path: &str) {
        let toml = toml::to_string(self).unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }

    /// Adds a new directory path to the paths config
    pub fn add_path(&mut self, path: &str) {
        if let Some(ref mut v) = self.paths {
            v.push(String::from(path));
        }
    }

    pub fn paths(&self) -> &Option<Vec<String>> {
        &self.paths
    }
}
