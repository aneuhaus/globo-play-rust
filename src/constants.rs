// src/constants.rs

// Base URLs
pub const PLAYBACK_API_BASE_URL: &str = "https://playback.video.globo.com";
pub const GRAPHQL_API_BASE_URL: &str = "https://cloud-jarvis.globo.com/graphql";
#[allow(dead_code)]
pub const THUMBNAIL_BASE_URL: &str = "https://s02.video.glbimg.com";

// URL Templates
pub const VIDEO_SESSION_URL_TEMPLATE: &str = "/v4/video-session";
#[allow(dead_code)]
pub const VIDEOS_BY_DATE_OPERATION: &str = "getTitleVideosByDateView";
#[allow(dead_code)]
pub const VIDEOS_BY_DATE_HASH: &str = "d4d95fd5770f9672dc1247e3343c13cafff725f339c95eb28c6e61dac9501c5d";
#[allow(dead_code)]
pub const VIDEO_DETAILS_URL_TEMPLATE: &str = "/videos/{}";

// Thumbnail resolution templates
#[allow(dead_code)]
pub const THUMBNAIL_SMALL_TEMPLATE: &str = "/x216/{}.jpg";
#[allow(dead_code)]
pub const THUMBNAIL_MEDIUM_TEMPLATE: &str = "/x720/{}.jpg";
#[allow(dead_code)]
pub const THUMBNAIL_LARGE_TEMPLATE: &str = "/x1080/{}.jpg";
