// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub duration: Option<u32>,
    pub headline: Option<String>,
    pub custom_id: Option<String>,
    pub resource_id: Option<String>, // Often used to get session info
    pub available_for: Option<String>,
    // Add other fields as discovered from API responses
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoResource {
    pub id: String,
    pub name: String,
    pub sources: Vec<Source>,
    pub session: Option<String>, // Session ID for streaming
    pub security_token: Option<String>,
    pub license_url: Option<String>,
    // Add other fields as discovered
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Source {
    #[serde(rename = "type")]
    pub type_: String, // "primary", "fallback"
    pub url: String,
    #[serde(default)]
    pub label: Option<String>, // Changed from String to Option<String>
    #[serde(default, alias = "sourceType")]
    pub source_type: String, 
    pub cdn: Option<String>, // CDN provider
    pub token: Option<String>, // Authentication token
    pub pop: Option<String>, // Point of presence
    pub asset_key: Option<String>, // Asset key
    pub expiration_time: Option<u64>, // Expiration timestamp
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoSession {
    #[serde(default)]
    pub session: String, // The actual session ID or key
    pub sources: Vec<Source>,
    pub resource: Option<VideoResourceDetails>, // Sometimes the resource details are nested
    pub metadata: Option<VideoMetadata>, // Metadata about the video
    pub thumbs_preview_base_url: Option<String>, // Preview thumbnails URL
    pub thumbs_url: Option<String> // Thumbnails URL
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoResourceDetails {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    // Potentially other details about the resource itself
}

// Model for a list of videos, as returned by date search or similar endpoints
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoItems {
    pub items: Vec<Video>,
    // Potentially pagination fields like next_page_token, has_next_page, etc.
}

// You might need more specific structs depending on the exact API responses.
// For example, if the /videos/by/date endpoint has a different structure:
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatedVideoItem {
    // Fields specific to the dated video list item
    pub id: String,
    pub title: String,
    pub date_formated: Option<String>,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub duration_formatted: Option<String>,
    pub duration_seconds: Option<u32>,
    pub custom_id: Option<String>,
    pub resource_id: Option<String>,
    pub video_url: Option<String>, // URL to the video page, not the stream itself
    // ... and so on
}

/// Comprehensive metadata about a video from the session API response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoMetadata {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub type_: Option<String>,
    #[serde(rename = "type")]
    pub video_type: String,
    pub duration: Option<u64>,
    pub program: Option<String>,
    pub program_id: Option<u64>,
    pub channel: Option<String>,
    pub channel_id: Option<u64>,
    pub category: Option<String>,
    pub created_at: Option<String>,
    pub exhibited_at: Option<String>,
    pub url_for_consumption: Option<String>,
    pub codec: Option<String>,
    pub max_height: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatedVideosResponse {
    pub items: Vec<DatedVideoItem>,
    pub count: Option<u32>,
    pub next: Option<String>, // URL for the next page of results
    // Other metadata related to the list
}

// Error structure for API responses
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiErrorResponse {
    pub message: String,
    pub code: Option<String>,
    // Other error details
}
