use crate::platform::Platform;
use reqwest::Url;
use serde::Deserialize;
use std::path::PathBuf;
use url::ParseError;

#[derive(Deserialize)]
struct Dependency {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct Java {
    name: String,
    url: String,
    main: String,
    
}

#[derive(Deserialize)]
struct Manifest {
    platform: Platform,
    java: Java,
    dependencies: Vec<Dependency>,
}

impl Dependency {

    fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.name)
    }

    fn get_url(&self) -> Result<Url, ParseError> {
        Url::parse(&self.url)
    }
}

impl Java {

    fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.name)
    }

    fn get_url(&self) -> Result<Url, ParseError> {
        Url::parse(&self.url)
    }
}