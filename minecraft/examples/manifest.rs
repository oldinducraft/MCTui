use std::path::Path;
use std::time::Duration;

use minecraft::manifest::Manifest;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let manifest_str = include_str!("example_manifest.json");
    let root = Path::new("./client");
    let manifest = Manifest::new(manifest_str, root).await.unwrap();
    let on_progress = |i, total| println!("Downloaded {i}/{total}");

    println!("Downloading game according to manifest...");
    let res = manifest.download(&on_progress).await;

    if res.is_err() {
        println!("Failed to download manifest, retying in 5 seconds...");
        sleep(Duration::from_secs(5)).await;
        manifest.download(&on_progress).await.unwrap();
    }

    println!("{:#?}", manifest);
}
