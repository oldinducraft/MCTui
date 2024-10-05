use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use futures::StreamExt;
use reqwest::Client;

use crate::constants::{CLIENT_ARCHIVE_FILENAME, CLIENT_URL, HASH_URL};

pub struct Requester {
    client: Client,
}

impl Requester {
    pub fn new() -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap(),
        }
    }

    pub async fn get_client_hash(&self) -> HashMap<String, String> {
        let hash = self
            .client
            .get(HASH_URL)
            .send()
            .await
            .unwrap_or_else(|err| panic!("Failed to get hash: {}", err))
            .text()
            .await
            .unwrap();

        serde_json::from_str::<HashMap<String, String>>(&hash)
            .unwrap_or_else(|err| panic!("Failed to parse hash: {}", err))
    }

    pub async fn download_client<F>(&self, mut on_progress: F)
    where
        F: FnMut(f64) + Send + 'static,
    {
        let res = self
            .client
            .get(CLIENT_URL)
            .send()
            .await
            .unwrap_or_else(|err| panic!("Failed to download client: {}", err));
        let total_size = res.content_length().expect("Failed to get client content length");

        let mut file =
            File::create(CLIENT_ARCHIVE_FILENAME).unwrap_or_else(|err| panic!("Failed to create client file: {}", err));
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(Ok(chunk)) = stream.next().await {
            file.write_all(&chunk).expect("Failed to write to client file");
            downloaded = min(downloaded + (chunk.len() as u64), total_size);

            let progress = downloaded as f64 / total_size as f64;
            on_progress(progress);
        }
    }
}
