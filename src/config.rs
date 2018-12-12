extern crate serde_derive;
extern crate toml;

use std::io::prelude::*;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    paths: Vec<PathBuf>,
}

impl Config {
    /// Creates a new empty config
    fn new() -> Config {
        Config { paths: vec![] }
    }

    /// Loads the config from file at given path, or creates a default config
    pub fn load<P: AsRef<Path>>(path: P) -> Config {
        // Load the file, or create a default Config if no file
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(..) => return Config::new(),
        };

        // If we loaded a file, read the file contents
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Deserialize the file as TOML into our Config struct
        toml::from_str(contents.as_ref()).unwrap()
    }

    /// Serializes the config and stores it to file at given path
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let toml = toml::to_string(self).unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }

    /// Adds a new directory path to the paths config
    pub fn add_path<P: AsRef<Path>>(&mut self, path: P) {
        self.paths.push(path.as_ref().to_owned())
    }

    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }
}
