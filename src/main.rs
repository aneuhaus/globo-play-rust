\
// src/main.rs

mod api;
mod cli;
mod config;
mod models;
mod utils;

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;
use cli::{Cli, Commands};
use config::AppConfig;
use models::{DatedVideoItem, Source, VideoSession};
use std::path::PathBuf;

fn select_best_stream(sources: &[Source], quality_preference: &str) -> Option<String> {
    // Simple quality selection logic, can be expanded
    // Assumes labels like "1080p", "720p", "480p", etc. or "max", "min"
    // For now, let's prioritize based on a predefined order or "max"
    if sources.is_empty() {
        return None;
    }

    let mut best_source: Option<&Source> = None;

    if quality_preference == "max" {
        // A simple way to get "max" could be to find the one with "p" and highest number
        // or just the first one if they are ordered by quality (often they are)
        best_source = sources.first(); // Assuming API returns them in some order
                                       // A more robust way would be to parse "1080p", "720p" etc.
    } else if quality_preference == "min" {
        best_source = sources.last();
    } else {
        for source in sources {
            if source.label.contains(quality_preference) {
                best_source = Some(source);
                break;
            }
        }
        // If specific quality not found, fallback to "max" or first available
        if best_source.is_none() {
            best_source = sources.first();
        }
    }
    best_source.map(|s| s.url.clone())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .replace(' ', "_")
}

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
                     println!("Title: {}", resource.name);
                     println!("ID: {}", resource.id);
                } else {
                    println!("Video ID: {}", video_id); // Fallback if resource details are not in session
                }
                println!("Available Streams:");
                for source in &session.sources {
                    println!("  - Label: {}, URL: {}", source.label, source.url);
                }
            }

            if download {
                let quality_pref = quality_override.as_ref().unwrap_or(&config.video_quality);
                if let Some(stream_url) = select_best_stream(&session.sources, quality_pref) {
                    let filename = custom_filename.unwrap_or_else(|| {
                        let title = session.resource.as_ref().map_or_else(
                            || video_id.clone(),
                            |r| sanitize_filename(&r.name),
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
                        stream_url,
                        download_path.display()
                    );
                    utils::download_file(&config.http_client, &stream_url, &download_path).await?;
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = AppConfig::from_cli(&cli).await.context("Failed to load application configuration")?;

    if config.debug_mode {
        println!("CLI args: {:?}", cli);
        println!("AppConfig: {:?}", config);
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
            // No subcommand was given.
            // clap will show help if no args are given or if --help is used.
            // If we want specific behavior when the app is run without subcommands, add it here.
            println!("No command provided. Use --help for options.");
            // For example, print version or a short usage message.
            // Cli::command().print_help()?; // This requires CommandFactory from clap
        }
    }

    Ok(())
}
