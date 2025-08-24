use reqwest::Client;
use std::io::Write;
use std::path::{Path, PathBuf};
use url::Url;
use zip::ZipArchive;

pub(crate) async fn download_jre(url: Url, path: &Path, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    if !path.is_dir() {
        let mut temporary = tempfile::NamedTempFile::new()?;

        let bytes = client.get(url).send().await?.bytes().await?;
        temporary.write_all(&*bytes)?;

        ZipArchive::new(&temporary)?.extract(path)?;
    }

    Ok(())
}