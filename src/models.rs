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
    pub label: String, // e.g., "720p", "1080p"
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoSession {
    pub session: String, // The actual session ID or key
    pub sources: Vec<Source>,
    pub resource: Option<VideoResourceDetails>, // Sometimes the resource details are nested
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoResourceDetails {
    pub id: String,
    pub name: String,
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
