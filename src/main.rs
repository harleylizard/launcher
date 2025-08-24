mod platform;
mod downloadable;
mod json;
mod jre;

use crate::downloadable::Downloadable;
use crate::jre::download_jre;
use crate::json::{Java, Manifest};
use crate::platform::get_platform;
use reqwest::Client;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::task::JoinSet;

fn relative(first: &Path, second: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let parent = first.parent().ok_or("No parent")?;

    Ok(parent.join(second))
}

async fn prompt_jre(java: &Java, path: &Path, client: Client) -> Result<(), Box<dyn Error>> {
    download_jre(java.get_url()?, path, &client).await?;
    
    Ok(())
}

async fn spin(path: &Path) -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string(path)?;

    let manifest: Manifest = serde_json::from_str(&string)?;
    if manifest.get_platform() != get_platform() {
        return Err(Box::<dyn Error>::from("Wrong platform"));
    }

    let client = Client::new();

    let mut tasks: JoinSet<()> = JoinSet::new();

    for dependency in manifest.get_dependencies() {
        let downloadable = Downloadable::new(relative(path, &dependency.get_path())?, dependency.get_url()?);

        let cl = client.clone();

        tasks.spawn(async move {
            downloadable.download(&cl).await.expect("Failed to download")
        });
    }

    while let Some(_task) = tasks.join_next().await {
    }

    let java = manifest.get_java();
    prompt_jre(java, &path, client).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let path = Path::new("manifest.json");
    if path.is_file() {
        spin(&path).await.expect("Failed to spin");
    }
}
