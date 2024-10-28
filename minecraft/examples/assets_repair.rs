use std::io::Write as _;
use std::path::Path;

use constants::RESOURCES_URL;
use minecraft::asset::Asset;
use minecraft::traits::concurrent_download::ConcurrentDownload;
use minecraft::traits::into_invalid::IntoInvalid;

#[tokio::main]
async fn main() {
    let assets_str = include_str!("example_assets.json");
    let assets: Vec<Asset> = serde_json::from_str(assets_str).unwrap();
    let root = Path::new("./client/assets/objects");

    let invalid_assets = assets.into_invalid(16, root).await.unwrap();
    println!("Found {} invalid assets", invalid_assets.len());
    invalid_assets
        .download_concurrently_if_needed(8, RESOURCES_URL, root, 5, &|i, total| {
            print!("\rDownloaded {}/{total}", i + 1);
            std::io::stdout().flush().unwrap()
        })
        .await
        .unwrap();

    println!();
}
