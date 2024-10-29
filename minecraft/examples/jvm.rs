use std::path::Path;

use constants::{LIBRARIES_URL, LOCAL_URL, RESOURCES_URL, YGGDRASIL_DOMAIN};
use minecraft::game_args::GameArgs;
use minecraft::hosts::Hosts;
use minecraft::jvm::Jvm;
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

    manifest
        .download_if_needed(&|i, total| println!("Downloading {i}/{total}"))
        .await
        .unwrap();

    let jvm = Jvm::new(&manifest, GameArgs::default(), YGGDRASIL_DOMAIN).await;
    println!("{}", jvm.command().join(" "))
}
