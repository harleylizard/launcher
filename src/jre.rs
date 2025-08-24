use reqwest::Client;
use std::io::Write;
use std::path::PathBuf;
use log::info;
use url::Url;
use zip::ZipArchive;

pub(crate) async fn download_jre(url: Url, path: &PathBuf, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    if !path.is_dir() {
        info!("Downloading {}", url);

        let mut temporary = tempfile::NamedTempFile::new()?;

        let bytes = client.get(url).send().await?.bytes().await?;
        temporary.write_all(&*bytes)?;

        ZipArchive::new(&temporary)?.extract(path)?;
    }

    Ok(())
}