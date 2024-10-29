use std::path::Path;

use constants::{LIBRARIES_URL, LOCAL_URL, RESOURCES_URL};
use minecraft::hosts::Hosts;
use minecraft::manifest::Manifest;

#[tokio::main]
async fn main() {
    let manifest_str = include_str!("example_manifest.json");
    let root = Path::new("./client");
    let manifest = Manifest::new(manifest_str, root, Hosts {
        misc:      LOCAL_URL.to_string(),
        resources: RESOURCES_URL.to_string(),
        libraries: LIBRARIES_URL.to_string(),
    })
    .await
    .unwrap();
    let on_progress = |i, total| println!("Downloaded {i}/{total}");

    println!("Downloading game according to manifest...");
    manifest.download_if_needed(&on_progress).await.unwrap();

    println!("{:#?}", manifest);
}
