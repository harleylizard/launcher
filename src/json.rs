use crate::platform::Platform;
use reqwest::Url;
use serde::Deserialize;
use std::path::PathBuf;
use url::ParseError;

#[derive(Deserialize)]
pub(crate) struct Dependency {
    name: String,
    url: String,
}

#[derive(Deserialize)]
pub(crate) struct Java {
    name: String,
    url: String,
    main: String,

}

#[derive(Deserialize)]
pub(crate) struct Manifest {
    platform: Platform,
    java: Java,
    dependencies: Vec<Dependency>,
}

impl Dependency {

    pub(crate) fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.name)
    }

    pub(crate) fn get_url(&self) -> Result<Url, ParseError> {
        Url::parse(&self.url)
    }
}

impl Java {

    pub(crate) fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.name)
    }

    pub(crate) fn get_url(&self) -> Result<Url, ParseError> {
        Url::parse(&self.url)
    }
}

impl Manifest {

    pub(crate) fn get_platform(&self) -> &Platform {
        &self.platform
    }

    pub(crate) fn get_java(&self) -> &Java {
        &self.java
    }

    pub(crate) fn get_dependencies(&self) -> &Vec<Dependency> {
        &self.dependencies
    }
}