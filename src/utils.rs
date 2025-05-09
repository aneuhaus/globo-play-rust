// src/utils.rs

use anyhow::{Context, Result};
use reqwest::Client; // Still useful for pre-checks
use std::path::Path;
use tokio::process::Command; // Changed to tokio::process::Command
use std::process::Stdio; // Added for piping ffmpeg output

// Basic file download utility using ffmpeg
// TODO: Add progress bar (ffmpeg output parsing can be complex).
// TODO: Check if ffmpeg is installed and provide a helpful error if not.
// TODO: Allow configuring ffmpeg path.
pub async fn download_file(client: &Client, url: &str, path: &Path) -> Result<()> {
    println!(
        "Attempting to download using ffmpeg. Input URL: \"{}\", Output Path: \"{}\"",
        url,
        path.display()
    );

    // 1. Preliminary HEAD request to check URL accessibility
    match client.head(url).send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                return Err(anyhow::anyhow!(
                    "HEAD request to URL {} failed with status: {}. Aborting ffmpeg download.",
                    url,
                    resp.status()
                ));
            }
            println!("URL {} is accessible (status: {}). Proceeding with ffmpeg.", url, resp.status());
        }
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to make HEAD request to URL {}: {}. Aborting ffmpeg download.",
                url,
                e
            ));
        }
    }

    // 2. Ensure the output directory exists
    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            tokio::fs::create_dir_all(parent_dir)
                .await
                .context(format!("Failed to create directory: {}", parent_dir.display()))?;
            println!("Created output directory: {}", parent_dir.display());
        }
    }

    let output_path_str = path.to_str().ok_or_else(|| {
        anyhow::anyhow!("Invalid output path for ffmpeg: {}", path.display())
    })?;

    // 3. Construct and execute ffmpeg command
    println!(
        "Executing ffmpeg command: ffmpeg -y -protocol_whitelist file,http,https,tcp,tls,crypto -i \"{}\" -c copy -bsf:a aac_adtstoasc \"{}\"",
        url, output_path_str
    );

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y") // Overwrite output files without asking
        .arg("-protocol_whitelist")
        .arg("file,http,https,tcp,tls,crypto")
        .arg("-i")
        .arg(url)
        .arg("-c")
        .arg("copy")
        .arg("-bsf:a")
        .arg("aac_adtstoasc")
        .arg(output_path_str)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = cmd.spawn().context(
        "Failed to spawn ffmpeg command. Is ffmpeg installed and in your PATH?",
    )?;

    // 4. Wait for the command to complete and capture output
    let output = child
        .wait_with_output()
        .await
        .context("Failed to wait for ffmpeg command execution")?;

    // 5. Check ffmpeg's exit status
    if output.status.success() {
        println!(
            "ffmpeg successfully downloaded {} to {}",
            url,
            path.display()
        );
        // Optionally print ffmpeg's stderr if it contains useful info (ffmpeg often uses stderr for progress/info)
        let stderr_output = String::from_utf8_lossy(&output.stderr);
        if !stderr_output.is_empty() {
            println!("ffmpeg stderr:\n{}", stderr_output);
        }
        Ok(())
    } else {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!(
            "ffmpeg command failed with status: {}.\\nInput URL: {}\\nOutput Path: {}\\n\\nffmpeg stdout:\\n{}\\n\\nffmpeg stderr:\\n{}",
            output.status,
            url,
            path.display(),
            stdout_str,
            stderr_str
        ))
    }
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
