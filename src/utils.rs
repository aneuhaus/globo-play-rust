// src/utils.rs

use anyhow::{Context, Result};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::io::AsyncWriteExt;

// Basic file download utility
// TODO: Add progress bar, better error handling, resumable downloads?
pub async fn download_file(client: &Client, url: &str, path: &Path) -> Result<()> {
    println!("Downloading {} to {}", url, path.display());

    let response = client.get(url).send().await.context("Failed to send download request")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Download request failed with status: {} for URL: {}",
            response.status(),
            url
        ));
    }

    let mut dest_file = tokio::fs::File::create(path)
        .await
        .context(format!("Failed to create file at {}", path.display()))?;

    let mut stream = response.bytes_stream();

    while let Some(item) = futures_util::StreamExt::next(&mut stream).await {
        let chunk = item.context("Error while downloading file chunk")?;
        dest_file
            .write_all(&chunk)
            .await
            .context(format!("Error writing to file {}", path.display()))?;
    }

    println!("Successfully downloaded {} to {}", url, path.display());
    Ok(())
}

// Helper for formatting output (JSON, pretty JSON, compact text)
// pub fn format_output<T: serde::Serialize>(
//     data: &T,
//     format_type: &str,
//     debug_mode: bool,
// ) -> Result<String> {
//     match format_type {
//         "json" => serde_json::to_string(data).context("Failed to serialize to JSON"),
//         "pretty" => serde_json::to_string_pretty(data).context("Failed to serialize to pretty JSON"),
//         "compact" => {
//             // Implement a compact text representation
//             // This will depend on the type T
//             // For now, just use debug print as a placeholder
//             Ok(format!("{:#?}", data))
//         }
//         _ => Ok(format!("{:#?}", data)), // Default to debug print
//     }
// }

// You might add other utilities here, like:
// - Date parsing and formatting
// - Cookie file reading/writing (if not handled entirely in config)
// - Specific text formatting for different output types
