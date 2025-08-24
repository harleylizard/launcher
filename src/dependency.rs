use reqwest::Url;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

struct Dependency {
    path: PathBuf,
    url: Url
}

impl Dependency {
    fn new(path: &Path, url: Url) -> Dependency {
        Dependency {
            path: path.to_path_buf(),
            url,
        }
    }

    async fn download(self) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.path.parent() {
            if !parent.is_dir() {
                fs::create_dir_all(parent)?;
            }
        }

        let bytes = reqwest::get(self.url).await?.bytes().await?;
        fs::write(self.path.as_path(), bytes)?;

        Ok(())
    }
}