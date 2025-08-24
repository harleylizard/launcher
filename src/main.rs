mod platform;
mod downloadable;
mod json;
mod jre;

use crate::downloadable::Downloadable;
use crate::jre::download_jre;
use crate::json::{Java, Manifest};
use crate::platform::get_platform;
use chrono::Local;
use console::Term;
use log::{error, info};
use reqwest::Client;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::task::JoinSet;

fn relative(first: &Path, second: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let parent = first.parent().ok_or("No parent")?;

    Ok(parent.join(second))
}

fn create_command(java: &Java) -> Command {
    let formatted = java.get_command();
    info!("{}", formatted);

    let runtime: Vec<_> = formatted.split_whitespace().collect();

    let mut command = Command::new(&runtime[0].trim());

    command.args(&runtime[1..]);

    command
}

async fn run_jre(java: &Java, client: Client) -> Result<(), Box<dyn Error>> {
    download_jre(java.get_url()?, &java.get_path(), &client).await?;

    let output = create_command(java).output()?;

    info!("{}", String::from_utf8_lossy(&output.stdout));
    error!("{}", String::from_utf8_lossy(&output.stderr));

    info!("{}", output.status.success());

    let term = Term::stdout();
    if term.is_term() {
        let mut buffer = [0; 1];

        io::stdin().read_exact(&mut buffer).await?;
    }

    Ok(())
}

async fn spin(path: &Path) -> Result<(), Box<dyn Error>> {
    fern::Dispatch::new()
        .filter(|metadata| {
            matches!(metadata.level(), log::Level::Error | log::Level::Info)
        })
        .chain(std::io::stdout())
        .format(|out, message, record| {
            out.finish(format_args!("{} {} {}", Local::now().format("%H:%M:%S"), record.level(), message))
        })
        .apply()?;

    let string = fs::read_to_string(path)?;

    let manifest: Manifest = serde_json::from_str(&string)?;
    if manifest.get_platform() != get_platform() {
        return Err(Box::<dyn Error>::from("Wrong platform"));
    }

    let client = Client::new();

    let mut tasks: JoinSet<()> = JoinSet::new();

    for dependency in manifest.get_dependencies() {
        let artifact = relative(path, &dependency.get_path())?;
        if !artifact.is_file() {
            let downloadable = Downloadable::new(artifact, dependency.get_url()?);

            let cl = client.clone();

            tasks.spawn(async move {
                downloadable.download(&cl).await.expect("Failed to download")
            });
        }
    }

    while let Some(_task) = tasks.join_next().await {
    }

    let java = manifest.get_java();
    run_jre(java, client).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let path = Path::new("manifest.json");
    if path.is_file() {
        spin(&path).await.expect("Failed to spin");
    }
}
