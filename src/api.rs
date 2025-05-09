// src/api.rs
use crate::config::AppConfig;
use crate::models::{ApiErrorResponse, DatedVideosResponse, VideoSession};
use crate::constants; // Add this line
use anyhow::{anyhow, Context, Result};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("HTTP error: {status} - {body}")]
    Http {
        status: StatusCode,
        body: String,
    },
    #[error("Failed to deserialize JSON response: {0}")]
    JsonDeserialization(#[source] serde_json::Error),
    #[error("API returned an error: {0}")]
    GloboApi(String),
}

async fn fetch_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    config: &AppConfig,
) -> Result<T, ApiError> {
    if config.debug_mode {
        println!("Fetching URL: {}", url);
    }

    let response = client.get(url).send().await.map_err(ApiError::Request)?;

    if config.debug_mode {
        println!("Response status: {}", response.status());
        // Potentially log headers if needed
    }

    let status = response.status();
    if status.is_success() {
        let text_body = response.text().await.map_err(ApiError::Request)?;
        if config.debug_mode {
            println!("Response body: {}", text_body);
        }
        serde_json::from_str::<T>(&text_body).map_err(|e| {
            if config.debug_mode {
                eprintln!("Failed to parse JSON: {}, body was: {}", e, text_body);
            }
            ApiError::JsonDeserialization(e)
        })
    } else {
        let text_body = response.text().await.map_err(ApiError::Request)?;
        if config.debug_mode {
            eprintln!("Error response body: {}", text_body);
        }
        // Try to parse Globo API error structure
        if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&text_body) {
            Err(ApiError::GloboApi(api_error.message))
        } else {
            Err(ApiError::Http {
                status,
                body: text_body,
            })
        }
    }
}

pub async fn fetch_video_session(
    video_id: &str,
    config: &AppConfig,
) -> Result<VideoSession, ApiError> {
    let url = format!(
        "{}{}",
        constants::GLOBO_API_BASE_URL,
        constants::VIDEO_SESSION_URL_TEMPLATE.replace("{}", video_id)
    );
    fetch_json(&config.http_client, &url, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn fetch_videos_by_date(
    title_id: &str,
    from_date: &str, // YYYY-MM-DD
    to_date: &str,   // YYYY-MM-DD
    page: u32,
    per_page: u32,
    config: &AppConfig,
) -> Result<DatedVideosResponse, ApiError> {
    // Based on gp-common-functions, params can include order, page, per_page, etc.
    // For now, sticking to what's in get-videos-by-date script
    let params = format!("page={}&per_page={}", page, per_page);
    let url = format!(
        "{}{}",
        constants::GLOBO_API_BASE_URL,
        constants::VIDEOS_BY_DATE_URL_TEMPLATE
            .replace("{}", title_id)
            .replacen("{}", from_date, 1)
            .replacen("{}", to_date, 1)
            .replacen("{}", &params, 1)
    );
    fetch_json(&config.http_client, &url, config).await
}

// Placeholder for fetching a single video's general info (not session)
// This might be useful if there's an endpoint for just metadata without sources.
// pub async fn fetch_video_details(video_id: &str, config: &AppConfig) -> Result<Video, ApiError> {
//     let url = format!(
//         "{}{}",
//         constants::GLOBO_API_BASE_URL,
//         constants::VIDEO_DETAILS_URL_TEMPLATE.replace("{}", video_id)
//     );
//     fetch_json(&config.http_client, &url, config).await
// }
