// src/constants.rs

pub const GLOBO_API_BASE_URL: &str = "https://globoapi.globo.com/globoplay/api";

// URL Templates
pub const VIDEO_SESSION_URL_TEMPLATE: &str = "/v2/videos/{}/session";
pub const VIDEOS_BY_DATE_URL_TEMPLATE: &str = "/programas/{}/videos/by/date/{}/{}/?{}";
pub const VIDEO_DETAILS_URL_TEMPLATE: &str = "/videos/{}";
