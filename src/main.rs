mod platform;
mod downloadable;
mod json;

use std::path::Path;
use reqwest::Client;

fn spin() {
    let client = Client::new();

}

fn main() {
    let path = Path::new("manifest.json");
    if path.is_file() {


    }

}
