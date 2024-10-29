use std::io::Write as _;
use std::path::Path;

use constants::RESOURCES_URL;
use minecraft::asset::Asset;
use minecraft::traits::concurrent_download::ConcurrentDownload;

#[tokio::main]
async fn main() {
    let assets_str = include_str!("example_assets.json");
    let assets: Vec<Asset> = serde_json::from_str(assets_str).unwrap();
    let root = Path::new("./client/assets/objects");
    let max_concurrent_downloads = 8;

    assets
        .download_concurrently_if_needed(max_concurrent_downloads, RESOURCES_URL, root, 5, &|i, total| {
            print!("\rDownloaded {i}/{total}");
            std::io::stdout().flush().unwrap()
        })
        .await
        .unwrap();
}
