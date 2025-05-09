// src/cli.rs

use clap::{Parser, Subcommand};

/// Globo Play API Tool - A comprehensive tool for interacting with Globo Play API in Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Path to cookie file for authentication
    #[clap(long, short, global = true)]
    pub cookie: Option<String>,

    /// Set video quality (low, medium, high, max)
    #[clap(long, global = true, default_value = "max")]
    pub quality: String,

    /// Output format (json, compact, pretty)
    #[clap(long, global = true, default_value = "pretty")]
    pub output: String,

    /// Enable debug mode
    #[clap(long, short, global = true)]
    pub debug: bool,

    /// Directory for downloaded videos
    #[clap(long, global = true, default_value = ".")]
    pub output_dir: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get basic info about a video
    Video {
        video_id: String,
        /// Download video(s)
        #[clap(long)]
        download: bool,
        /// Custom filename for downloaded video
        #[clap(long)]
        filename: Option<String>,
        /// Set video quality (low, medium, high, max) - overrides global
        #[clap(long)]
        quality: Option<String>,
        /// Directory for downloaded videos - overrides global
        #[clap(long)]
        output_dir: Option<String>,
    },
    /// Get detailed info with sources
    VideoInfo {
        video_id: String,
        /// Download video(s)
        #[clap(long)]
        download: bool,
        /// Custom filename for downloaded video
        #[clap(long)]
        filename: Option<String>,
        /// Set video quality (low, medium, high, max) - overrides global
        #[clap(long)]
        quality: Option<String>,
        /// Directory for downloaded videos - overrides global
        #[clap(long)]
        output_dir: Option<String>,
    },
    /// Get videos by date range
    VideosByDate {
        title_id: String,
        from_date: Option<String>, // Optional, will use default if not provided
        to_date: Option<String>,   // Optional, will use default if not provided (or same as from_date)
        /// Download all fetched videos
        #[clap(long)]
        download_all: bool,
    },
}

// Functions to handle commands will go here or in main.rs
// pub async fn handle_command(cli: Cli, config: config::Config) -> anyhow::Result<()> {
//     match cli.command {
//         Some(Commands::Video { video_id, download, filename, quality, output_dir }) => {
//             println!("Video command: {}, download: {}, filename: {:?}", video_id, download, filename);
//             // Call video fetching logic
//         }
//         Some(Commands::VideoInfo { video_id, download, filename, quality, output_dir }) => {
//             println!("Video Info command: {}, download: {}, filename: {:?}", video_id, download, filename);
//             // Call video info fetching logic
//         }
//         Some(Commands::VideosByDate { title_id, from_date, to_date, download_all }) => {
//             println!("Videos by Date command: {}, from: {:?}, to: {:?}, download_all: {}", title_id, from_date, to_date, download_all);
//             // Call videos by date fetching logic
//         }
//         None => {
//             // Show help if no command is provided, or define default behavior
//             println!("No command provided. Use --help for options.");
//         }
//     }
//     Ok(())
// }
