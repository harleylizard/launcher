use reqwest::Client;
use reqwest::Url;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub(crate) struct Downloadable {
    path: PathBuf,
    url: Url
}

impl Downloadable {
    pub(crate) fn new(path: PathBuf, url: Url) -> Downloadable {
        Downloadable {
            path,
            url,
        }
    }

    pub(crate) async fn download(self, client: &Client) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let bytes = client.get(self.url).send().await?.bytes().await?;
        fs::write(self.path.as_path(), bytes)?;

        Ok(())
    }
}