// src/main.rs

mod api;
mod cli;
mod config;
mod models;
mod utils;
mod constants;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use config::AppConfig;
use models::Source;
use std::path::PathBuf;

/// Selects the best stream source based on the specified quality preference.
/// 
/// # Arguments
/// * `sources` - A slice of available video sources
/// * `quality_preference` - Quality preference ("max", "min", or specific quality like "720p")
///
/// # Returns
/// Option containing the URL of the selected source, or None if no sources available
fn select_best_stream(sources: &[Source], quality_preference: &str, cli_quality_arg: Option<&str>) -> Option<Source> {
    if sources.is_empty() {
        return None;
    }

    // Attempt 1: Exact match on label based on quality_preference (e.g., "1080p")
    let exact_match = sources.iter().find(|s| {
        s.label.as_ref().map_or(false, |lbl| !lbl.is_empty() && lbl.contains(quality_preference))
    });
    if exact_match.is_some() {
        return exact_match.cloned();
    }

    // Attempt 2: If CLI quality is "high" or "low", use dedicated functions
    if let Some(cli_quality) = cli_quality_arg {
        if cli_quality == "high" {
            return find_highest_quality_source(sources, true);
        } else if cli_quality == "low" {
            return find_lowest_quality_source(sources, true);
        }
    } 

    // Attempt 3: Default to primary source or first source
    let primary_source = sources.iter().find(|s| 
        s.source_type.as_str() == "primary" || 
        s.type_.as_str() == "primary"
    );

    primary_source.cloned().or_else(|| sources.first().cloned())
}

/// Finds the highest quality source from a list of sources
/// 
/// Attempts to parse resolution values like "1080p", "720p", etc. or uses metadata
fn find_highest_quality_source(sources: &[Source], prefer_primary: bool) -> Option<Source> {
    let mut best_source: Option<Source> = None;
    let mut max_resolution: u32 = 0;

    let filtered_sources = sources.iter().filter(|s| {
        if prefer_primary {
            // Prefer "source_type" if available, otherwise fallback to "type"
            s.source_type.as_str() == "primary" || s.type_.as_str() == "primary"
        } else {
            true // Consider all sources if not preferring primary
        }
    });

    for source in filtered_sources {
        let mut current_resolution: Option<u32> = None;

        if let Some(label_str) = source.label.as_deref() {
            if !label_str.is_empty() {
                if let Some(res) = extract_resolution(label_str) {
                    current_resolution = Some(res);
                }
            }
        }

        if current_resolution.is_none() {
            if let Some(res) = extract_resolution_from_url(&source.url) {
                current_resolution = Some(res);
            }
        }

        if current_resolution.is_none() {
            if let Some(asset_key_str) = source.asset_key.as_deref() {
                 if !asset_key_str.is_empty() {
                    if let Some(res) = extract_resolution(asset_key_str) { // asset_key can also contain resolution info
                        current_resolution = Some(res);
                    }
                }
            }
        }

        if let Some(resolution) = current_resolution {
            if resolution > max_resolution {
                max_resolution = resolution;
                best_source = Some(source.clone());
            }
        }
    }

    best_source
}

/// Finds the lowest quality source from a list of sources
fn find_lowest_quality_source(sources: &[Source], prefer_primary: bool) -> Option<Source> {
    let mut worst_source: Option<Source> = None;
    let mut min_resolution: u32 = u32::MAX;

    let filtered_sources = sources.iter().filter(|s| {
        if prefer_primary {
            // Prefer "source_type" if available, otherwise fallback to "type"
            s.source_type.as_str() == "primary" || s.type_.as_str() == "primary"
        } else {
            true // Consider all sources if not preferring primary
        }
    });

    for source in filtered_sources {
        let mut current_resolution: Option<u32> = None;

        if let Some(label_str) = source.label.as_deref() {
            if !label_str.is_empty() {
                if let Some(res) = extract_resolution(label_str) {
                    current_resolution = Some(res);
                }
            }
        }
        
        if current_resolution.is_none() {
            if let Some(res) = extract_resolution_from_url(&source.url) {
                current_resolution = Some(res);
            }
        }

        if current_resolution.is_none() {
            if let Some(asset_key_str) = source.asset_key.as_deref() {
                if !asset_key_str.is_empty() {
                    if let Some(res) = extract_resolution(asset_key_str) { // asset_key can also contain resolution info
                        current_resolution = Some(res);
                    }
                }
            }
        }

        if let Some(resolution) = current_resolution {
            if resolution < min_resolution {
                min_resolution = resolution;
                worst_source = Some(source.clone());
            }
        }
    }

    worst_source
}

/// Extracts resolution value from labels like "720p", "1080p HD", etc.
fn extract_resolution(label: &str) -> Option<u32> {
    // Find digits followed by 'p'
    let re = regex::Regex::new(r"(\d+)p").ok()?;
    re.captures(label)
        .and_then(|caps| caps.get(1))
        .and_then(|res| res.as_str().parse::<u32>().ok())
}

/// Extracts resolution value from URLs containing resolution information
/// 
/// For example: urls containing "r360_1080" would extract 1080
fn extract_resolution_from_url(url: &str) -> Option<u32> {
    // Look for common resolution patterns in URLs
    let res_patterns = [
        // r360_1080 pattern
        regex::Regex::new(r"r\d+_(\d+)").ok()?,
        // direct resolution like 1080p or 720p
        regex::Regex::new(r"(\d+)p").ok()?,
        // Any sequence of 3-4 digits that might represent resolution
        regex::Regex::new(r"/(\d{3,4})(/|_|\.)").ok()?
    ];
    
    for pattern in &res_patterns {
        if let Some(caps) = pattern.captures(url) {
            if let Some(res_match) = caps.get(1) {
                if let Ok(res) = res_match.as_str().parse::<u32>() {
                    return Some(res);
                }
            }
        }
    }
    
    None
}

/// Sanitizes a string to be used as a valid filename
///
/// Removes special characters and replaces spaces with underscores
///
/// # Arguments
/// * `name` - The string to sanitize
///
/// # Returns
/// A sanitized string suitable for use as a filename
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .replace(' ', "_")
}

/// Handles the video command, fetching video information and optionally downloading the video
///
/// # Arguments
/// * `video_id` - The ID of the video to fetch
/// * `download` - Whether to download the video
/// * `custom_filename` - Optional custom filename for the downloaded video
/// * `quality_override` - Optional quality override for the video
/// * `output_dir_override` - Optional output directory for the downloaded video
/// * `config` - The application configuration
/// * `fetch_full_info` - Whether to fetch full video info (true) or basic info (false)
///
/// # Returns
/// Result indicating success or error
async fn handle_video_command(
    video_id: String,
    download: bool,
    custom_filename: Option<String>,
    quality_override: Option<String>,
    output_dir_override: Option<String>,
    config: &AppConfig,
    fetch_full_info: bool, // True for VideoInfo, false for Video (basic)
) -> Result<()> {
    println!("Fetching video session for ID: {}", video_id);
    match api::fetch_video_session(&video_id, config).await {
        Ok(session) => {
            if fetch_full_info || config.output_format == "json" || config.output_format == "pretty" {
                let output_str = if config.output_format == "pretty" {
                    serde_json::to_string_pretty(&session)?
                } else {
                    serde_json::to_string(&session)?
                };
                println!("{}", output_str);
            } else {
                // Compact output for basic video info
                if let Some(resource) = &session.resource {
                     println!("Title: {}", resource.name.as_deref().unwrap_or("N/A"));
                     println!("ID: {}", resource.id.as_deref().unwrap_or("N/A"));
                } else {
                    println!("Video ID: {}", video_id); // Fallback if resource details are not in session
                }
                println!("Available Streams:");
                for source in &session.sources {
                    println!("  - Label: {}, URL: {}", source.label.as_deref().unwrap_or("N/A"), source.url);
                }
            }

            if download {
                let quality_pref = quality_override.as_ref().unwrap_or(&config.video_quality);
                // Pass the cli_quality_arg to select_best_stream
                let cli_quality_arg = quality_override.as_deref(); 
                if let Some(stream_source) = select_best_stream(&session.sources, quality_pref, cli_quality_arg) {
                    let filename = custom_filename.unwrap_or_else(|| {
                        let title = session.resource.as_ref().map_or_else(
                            || video_id.clone(),
                            |r| sanitize_filename(r.name.as_deref().unwrap_or(&video_id)),
                        );
                        format!("{}.mp4", title) // Assuming mp4, might need to check source type
                    });

                    let output_dir = output_dir_override
                        .map(PathBuf::from)
                        .unwrap_or_else(|| config.download_dir.clone());
                    let mut download_path = output_dir;
                    download_path.push(filename);

                    println!(
                        "Downloading video from {} to {}",
                        stream_source.url, // Use stream_source.url instead of stream_source
                        download_path.display()
                    );
                    utils::download_file(&config.http_client, &stream_source.url, &download_path).await?; // Use &stream_source.url
                    println!("Download complete: {}", download_path.display());
                } else {
                    eprintln!("Could not find a suitable stream to download for quality preference: {}", quality_pref);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching video session for {}: {}", video_id, e);
            return Err(e.into());
        }
    }
    Ok(())
}

/// Handles fetching videos by date and optionally downloading all videos in the result
///
/// # Arguments
/// * `title_id` - The ID of the title/program to fetch videos for
/// * `from_date_opt` - Optional start date (format: YYYY-MM-DD)
/// * `to_date_opt` - Optional end date (format: YYYY-MM-DD)
/// * `download_all` - Whether to download all videos in the result
/// * `config` - The application configuration
///
/// # Returns
/// Result indicating success or error
async fn handle_videos_by_date_command(
    title_id: String,
    from_date_opt: Option<String>,
    to_date_opt: Option<String>,
    download_all: bool,
    config: &AppConfig,
) -> Result<()> {
    let today = chrono::Local::now().date_naive();
    let from_date = from_date_opt.unwrap_or_else(|| today.format("%Y-%m-%d").to_string());
    let to_date = to_date_opt.unwrap_or_else(|| from_date.clone()); // Default to_date to from_date if not specified

    // For simplicity, fetch first page, 20 items. Pagination can be added later.
    let page = 1;
    let per_page = 20;

    println!(
        "Fetching videos for title ID: {} from {} to {} (page {}, per_page {})",
        title_id, from_date, to_date, page, per_page
    );

    match api::fetch_videos_by_date(&title_id, &from_date, &to_date, page, per_page, config).await {
        Ok(response) => {
            if config.output_format == "pretty" {
                println!("{}", serde_json::to_string_pretty(&response.items)?);
            } else if config.output_format == "json" {
                println!("{}", serde_json::to_string(&response.items)?);
            } else {
                // Compact output
                println!("Found {} videos:", response.items.len());
                for video_item in &response.items {
                    println!(
                        "  ID: {}, Title: {}, Date: {}",
                        video_item.id,
                        video_item.headline.as_deref().unwrap_or("N/A"),
                        video_item.date_formated.as_deref().unwrap_or("N/A")
                    );
                }
            }

            if download_all {
                if response.items.is_empty() {
                    println!("No videos found to download.");
                    return Ok(());
                }
                println!("Attempting to download all {} videos...", response.items.len());
                for video_item in response.items {
                    let video_id_to_download = video_item.resource_id.as_ref().unwrap_or(&video_item.id);
                    println!("--- Downloading video: {} ({}) ---", video_item.headline.as_deref().unwrap_or("N/A"), video_id_to_download);
                    // Use default quality and output dir from global config for batch downloads
                    // Filename will be auto-generated based on title
                    if let Err(e) = handle_video_command(
                        video_id_to_download.clone(),
                        true,
                        None, // No custom filename for batch
                        None, // Use global quality
                        None, // Use global output dir
                        config,
                        false, // Don't need full info print during batch download
                    ).await {
                        eprintln!("Failed to download video {}: {}", video_id_to_download, e);
                        // Continue with the next video
                    }
                    println!("--------------------------------------");
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching videos by date for {}: {}", title_id, e);
            return Err(e.into());
        }
    }
    Ok(())
}

/// Main entry point for the application
#[tokio::main]
async fn main() -> Result<()> {
    // Display welcome banner
    let version = env!("CARGO_PKG_VERSION");
    println!("Globo Play Rust v{} - Command-line utility", version);
    println!("----------------------------------------");

    let cli = Cli::parse();
    let config = AppConfig::from_cli(&cli).await.context("Failed to load application configuration")?;

    if config.debug_mode {
        println!("DEBUG: CLI args: {:?}", cli);
        println!("DEBUG: AppConfig: {:?}", config);
    }

    match cli.command {
        Some(Commands::Video {
            video_id,
            download,
            filename,
            quality,
            output_dir,
        }) => {
            handle_video_command(video_id, download, filename, quality, output_dir, &config, false).await?
        }
        Some(Commands::VideoInfo {
            video_id,
            download,
            filename,
            quality,
            output_dir,
        }) => {
            handle_video_command(video_id, download, filename, quality, output_dir, &config, true).await?
        }
        Some(Commands::VideosByDate {
            title_id,
            from_date,
            to_date,
            download_all,
        }) => {
            handle_videos_by_date_command(title_id, from_date, to_date, download_all, &config).await?
        }
        None => {
            // No subcommand was given
            println!("No command provided. Here are some examples to get you started:");
            println!();
            println!("  # Get information about a specific video:");
            println!("  globo_play_rust video VIDEO_ID");
            println!();
            println!("  # Download a specific video with highest quality:");
            println!("  globo_play_rust video VIDEO_ID --download");
            println!();
            println!("  # Get videos by date range for a specific title/program:");
            println!("  globo_play_rust videos-by-date TITLE_ID --from-date 2023-01-01 --to-date 2023-01-31");
            println!();
            println!("For more options, use --help:");
            println!("  globo_play_rust --help");
            println!("  globo_play_rust video --help");
        }
    }

    Ok(())
}
