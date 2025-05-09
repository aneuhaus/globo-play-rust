// src/api.rs
use crate::config::AppConfig;
use crate::models::{ApiErrorResponse, DatedVideosResponse, VideoSession};
use crate::constants;
use anyhow::Result;
use reqwest::StatusCode;
use thiserror::Error;
use uuid;
use urlencoding;

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

pub async fn fetch_video_session(
    video_id: &str,
    config: &AppConfig,
) -> Result<VideoSession, ApiError> {
    let url = format!("{}{}", constants::PLAYBACK_API_BASE_URL, constants::VIDEO_SESSION_URL_TEMPLATE);
    
    if config.debug_mode {
        println!("Fetching video session for ID: {}", video_id);
        println!("URL: {}", url);
    }
    
    // Following the pattern from marine-traffic/gp-common-functions
    let vsid = uuid::Uuid::new_v4().to_string();
    let request_body = serde_json::json!({
        "player_type": "desktop",
        "video_id": video_id,
        "quality": config.video_quality,
        "content_protection": "widevine",
        "vsid": vsid,
        "tz": "-03:00",
        "capabilities": {
            "low_latency": true
        },
        "consumption": "streaming",
        "metadata": {
            "name": "web",
            "device": {
                "type": "desktop",
                "os": {}
            }
        },
        "version": 1
    });
    
    let response = config.http_client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(ApiError::Request)?;
        
    let status = response.status();
    if status.is_success() {
        let text_body = response.text().await.map_err(ApiError::Request)?;
        if config.debug_mode {
            println!("Response body: {}", text_body);
        }
        serde_json::from_str::<VideoSession>(&text_body).map_err(|e| {
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

#[allow(clippy::too_many_arguments)]
pub async fn fetch_videos_by_date(
    title_id: &str,
    from_date: &str, // YYYY-MM-DD
    to_date: &str,   // YYYY-MM-DD
    page: u32,
    per_page: u32,
    config: &AppConfig,
) -> Result<DatedVideosResponse, ApiError> {
    // Build GraphQL request based on get-videos-by-date script
    let operation_name = "getTitleVideosByDateView";
    let query_hash = "d4d95fd5770f9672dc1247e3343c13cafff725f339c95eb28c6e61dac9501c5d";
    
    // Build variables JSON
    let variables = serde_json::json!({
        "titleId": title_id,
        "gte": from_date,
        "lte": to_date,
        "page": page,
        "perPage": per_page
    });
    
    // Build extensions JSON
    let extensions = serde_json::json!({
        "persistedQuery": {
            "version": 1,
            "sha256Hash": query_hash
        }
    });
    
    // URL encode parameters for URL
    let variables_string = variables.to_string();
    let extensions_string = extensions.to_string();
    let encoded_variables = urlencoding::encode(&variables_string);
    let encoded_extensions = urlencoding::encode(&extensions_string);
    
    // Construct the URL
    let url = format!(
        "{}?operationName={}&variables={}&extensions={}",
        constants::GRAPHQL_API_BASE_URL,
        operation_name,
        encoded_variables,
        encoded_extensions
    );
    
    if config.debug_mode {
        println!("GraphQL request URL: {}", url);
    }
    
    // Make the request with appropriate headers
    let response = config.http_client
        .get(&url)
        .header("x-tenant-id", "globo-play")
        .header("x-platform-id", "web")
        .header("x-device-id", "desktop")
        .send()
        .await
        .map_err(ApiError::Request)?;
    
    let status = response.status();
    if !status.is_success() {
        let text_body = response.text().await.map_err(ApiError::Request)?;
        return Err(ApiError::Http {
            status,
            body: text_body,
        });
    }
    
    // Parse the GraphQL response format, which is different from the API response
    let text_body = response.text().await.map_err(ApiError::Request)?;
    if config.debug_mode {
        println!("GraphQL response: {}", text_body);
    }
    
    // First parse the outer GraphQL structure
    let graphql_response: serde_json::Value = serde_json::from_str(&text_body)
        .map_err(ApiError::JsonDeserialization)?;
    
    // Extract the data.title.structure.excerpts.resources array
    let resources = graphql_response
        .get("data")
        .and_then(|data| data.get("title"))
        .and_then(|title| title.get("structure"))
        .and_then(|structure| structure.get("excerpts"))
        .and_then(|excerpts| excerpts.get("resources"))
        .ok_or_else(|| ApiError::GloboApi("Missing resources in GraphQL response".to_string()))?;
    
    // Convert to our DatedVideosResponse format
    let resources_json = serde_json::to_string(resources)
        .map_err(|e| ApiError::JsonDeserialization(e))?;
    
    let videos_response: DatedVideosResponse = serde_json::from_str(&resources_json)
        .map_err(|e| ApiError::JsonDeserialization(e))?;
    
    Ok(videos_response)
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
