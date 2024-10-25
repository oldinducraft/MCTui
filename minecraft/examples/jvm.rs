use std::path::Path;

use minecraft::game_args::GameArgs;
use minecraft::jvm::Jvm;
use minecraft::manifest::Manifest;

#[tokio::main]
async fn main() {
    let manifest_str = include_str!("example_manifest.json");
    let root = Path::new("./client");
    let manifest = Manifest::new(manifest_str, root).await.unwrap();

    manifest.download(&|i, total| println!("Downloading {i}/{total}")).await.unwrap();

    let jvm = Jvm::new(&manifest, GameArgs::default()).await;
    println!("{}", jvm.command().join(" "))
}
