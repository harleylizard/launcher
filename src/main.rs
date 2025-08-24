mod platform;
mod downloadable;
mod json;

use crate::json::Manifest;
use crate::platform::get_platform;
use reqwest::Client;
use std::error::Error;
use std::fs;
use std::path::Path;
use tokio::task::JoinSet;
use crate::downloadable::Downloadable;

async fn spin(path: &Path) -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string(path)?;

    let manifest: Manifest = serde_json::from_str(&string)?;
    if manifest.get_platform() != get_platform() {
        return Err(Box::<dyn Error>::from("Wrong platform"));
    }

    let mut tasks: JoinSet<()> = JoinSet::new();

    let client = Client::new();
    for dependency in manifest.get_dependencies() {
        let file = path.parent().ok_or("No parent")?.join(dependency.get_path());

        let downloadable = Downloadable::new(file, dependency.get_url()?);

        let clone = client.clone();
        tasks.spawn(async move {
            downloadable.download(&clone).await.expect("Download failed");
        });
    }

    while let Some(_task) = tasks.join_next().await {
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let path = Path::new("manifest.json");
    if path.is_file() {
        spin(&path).await.expect("Failed to spin");
    }
}
