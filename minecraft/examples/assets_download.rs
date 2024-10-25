use std::io::Write as _;
use std::path::Path;

use minecraft::manifest::download::{Download, Resource};
use minecraft::traits::download::Download as _;

#[tokio::main]
async fn main() {
    let assets_str = include_str!("example_assets.json");
    let assets: Vec<Download<Resource>> = serde_json::from_str(assets_str).unwrap();
    let root = Path::new("./client/assets/objects");

    for (i, asset) in assets.iter().enumerate() {
        let msg = format!("Downloading {}/{}...", i, assets.len());
        print!("{msg}");
        asset
            .download(root, &move |downloaded, total_size| {
                print!("\r{msg} {:.0}%", downloaded as f64 / total_size as f64 * 100.0);
                std::io::stdout().flush().unwrap();
            })
            .await
            .unwrap();

        println!();
    }
}
