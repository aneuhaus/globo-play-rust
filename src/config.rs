// src/config.rs
use crate::cli::Cli;
use anyhow::Result;
use serde::Deserialize;
use shellexpand;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ConfigFile {
    pub cookie_file: Option<String>,
    pub default_quality: Option<String>,
    pub default_output_format: Option<String>,
    pub default_download_dir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    #[allow(dead_code)]
    pub cookie_file_path: Option<PathBuf>,
    pub video_quality: String,
    pub output_format: String,
    pub debug_mode: bool,
    pub download_dir: PathBuf,
    pub http_client: reqwest::Client,
}

impl AppConfig {
    pub async fn from_cli(cli: &Cli) -> Result<Self> {
        // Attempt to load config from a file (e.g., ~/.config/globo-play-rust/config.toml)
        // For simplicity, we'll skip the config file loading for now and use CLI args or defaults.

        let cookie_file_path = cli
            .cookie
            .as_ref()
            .map(|p| PathBuf::from(shellexpand::tilde(p).into_owned()));

        let download_dir = PathBuf::from(shellexpand::tilde(&cli.output_dir).into_owned());
        if !download_dir.exists() {
            fs::create_dir_all(&download_dir)?;
        }

        // Initialize HTTP client with cookie store
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"),
        );
        
        // Add additional headers found in marine-traffic scripts
        headers.insert(
            reqwest::header::ORIGIN,
            reqwest::header::HeaderValue::from_static("https://globoplay.globo.com"),
        );
        headers.insert(
            reqwest::header::REFERER,
            reqwest::header::HeaderValue::from_static("https://globoplay.globo.com/"),
        );
        headers.insert(
            "x-platform-id",
            reqwest::header::HeaderValue::from_static("web"),
        );
        headers.insert(
            "x-device-id",
            reqwest::header::HeaderValue::from_static("desktop"),
        );

        let cookie_store = reqwest::cookie::Jar::default();
        if let Some(ref path) = cookie_file_path {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                for line in content.lines() {
                    if line.starts_with("#") || line.trim().is_empty() {
                        continue;
                    }
                    let parts: Vec<&str> = line.split('\t').collect();
                    if parts.len() >= 7 {
                        // Basic Netscape cookie format parsing
                        // This is a simplified parser. A more robust one might be needed.
                        let domain = parts[0];
                        // let _flag = parts[1]; // TRUE/FALSE - path accessible from all paths
                        let _path_str = parts[2];
                        // let _secure = parts[3]; // TRUE/FALSE
                        // let _expiration = parts[4];
                        let name = parts[5];
                        let value = parts[6];

                        let cookie_str = format!("{}={}", name, value);
                        // The reqwest::cookie::Jar needs a URL to associate the cookie with.
                        // We'll use a placeholder Globo.com URL.
                        // This might need adjustment based on actual cookie requirements.
                        let url = format!("https://{}/", domain.trim_start_matches('.'))
                            .parse::<reqwest::Url>()?;
                        cookie_store.add_cookie_str(&cookie_str, &url);
                    }
                }
            }
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_provider(std::sync::Arc::new(cookie_store))
            .build()?;

        Ok(AppConfig {
            cookie_file_path,
            video_quality: cli.quality.clone(),
            output_format: cli.output.clone(),
            debug_mode: cli.debug,
            download_dir,
            http_client: client,
        })
    }
}

// Placeholder for loading from a config file, not used in this iteration
// pub fn load_config_from_file(path: &PathBuf) -> Result<Option<ConfigFile>> {
//     if path.exists() {
//         let content = fs::read_to_string(path)?;
//         let config: ConfigFile = toml::from_str(&content)?;
//         Ok(Some(config))
//     } else {
//         Ok(None)
//     }
// }
