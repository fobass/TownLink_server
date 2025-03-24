use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbyEventsRequest {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default = "default_radius")]
    pub radius: f64,  // Radius in kilometers
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub category: Option<String>,
    pub sort_by: Option<String>,
}

fn default_radius() -> f64 {
    10000.0 // Default radius in km
}

fn default_page() -> i32 {
    1
}

fn default_limit() -> i32 {
    20
}


#[derive(Debug,Deserialize)]
pub struct LoginQuery {
    pub provider: String,
    pub access_token: String,
    pub lat: f64,
    pub lon: f64,
}