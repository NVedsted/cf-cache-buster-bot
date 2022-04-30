use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CFError {
    pub code: usize,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CFResponse {
    pub success: bool,
    pub errors: Vec<CFError>,
}

pub async fn purge_file_cache(service_token: &str, zone_identifier: &str, url: &str) -> reqwest::Result<CFResponse> {
    let client = reqwest::Client::new();
    let response = client.post(format!("https://api.cloudflare.com/client/v4/zones/{}/purge_cache", zone_identifier))
        .header("X-Auth-User-Service-Key", service_token)
        .json(&[url])
        .send()
        .await?;

    response.json().await
}
