use std::error::Error;
use std::net::IpAddr;

use serde::Deserialize;

#[derive(Deserialize)]
struct IpResponse {
    origin: String,
}

const WEBSITE_URL: &'static str = "https://httpbin.org/ip";

/// Checks the current WAN IP.
///
/// Connects to WEBSITE_URL and retrieves the current WAN IP value.
async fn check_current_ip() -> Result<IpAddr, Box<dyn Error>> {
    const WEBSITE_URL: &'static str = "https://httpbin.org/ip";
    let resp = reqwest::get(WEBSITE_URL)
        .await?
        .json::<IpResponse>()
        .await?;
    let ip = resp.origin.parse::<IpAddr>()?;

    Ok(ip)
}
