mod platform;

use std::path::Path;

fn main() {
    let path = Path::new("manifest.json");
    if path.is_file() {

        return;
    }

}
