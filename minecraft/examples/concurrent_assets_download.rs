use std::io::Write as _;
use std::path::Path;

use minecraft::manifest::download::{Download, Resource};
use minecraft::traits::concurrent_download::ConcurrentDownload;

#[tokio::main]
async fn main() {
    let assets_str = include_str!("example_assets.json");
    let assets: Vec<Download<Resource>> = serde_json::from_str(assets_str).unwrap();
    let root = Path::new("./client/assets/objects");
    let max_concurrent_downloads = 8;

    assets
        .download_concurrently(
            max_concurrent_downloads,
            root,
            5,
            &|i, total| {
                print!("\rDownloaded {i}/{total}");
                std::io::stdout().flush().unwrap()
            },
        )
        .await
        .unwrap();
}
