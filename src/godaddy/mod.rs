use crate::dns::DNSRecord;

pub mod api;

use log::{debug, info};

use crate::auth::Auth;
use api::{Api, ResponseError};
use reqwest::Response;

/// Sends a put request to the GoDaddy API to update a DNS record.
pub async fn update_record(record: DNSRecord, auth: &Auth) -> () {
    let api: Api = Api::Patch(record.clone());

    let body = vec![record.clone()];
    debug!("{:?}", body);

    let header = auth.as_header();

    let client = reqwest::Client::new();

    let req = client
        .patch(api.to_string())
        .json(&body)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .header("authorization", &header);
    debug!("{:?}", api.to_string());

    let response = req.send().await.expect("Error updating records.");
    parse_response(response, record).await;
}

pub async fn delete_record(record: DNSRecord, auth: &Auth) -> () {
    let api: Api = Api::Delete(record.clone());

    let header = auth.as_header();

    let client = reqwest::Client::new();

    let req = client
        .delete(api.to_string())
        .header("accept", "application/json")
        .header("authorization", &header);

    let response = req.send().await.expect("Error deleting record.");
    parse_response(response, record).await;
}

async fn parse_response(response: Response, record: DNSRecord) -> () {
    match response.status() {
        s if s.is_success() => {
            info!("success: {:?}", record);
        }
        s if s.is_client_error() => {
            let body = response.text().await.unwrap();
            match serde_json::from_str::<ResponseError>(&body) {
                Ok(json) => {
                    info!("client error [{}]: {}\n{:?}", s, json.message, json.fields);
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {:?}", e);
                    eprintln!("Raw response body: {:?}", body);
                }
            };
        }
        s => {
            let body = response.text().await.unwrap();
            match serde_json::from_str::<ResponseError>(&body) {
                Ok(json) => {
                    info!("client error [{}]: {}\n{:?}", s, json.message, json.fields);
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {:?}", e);
                    eprintln!("Raw response body: {:?}", body);
                }
            }
        }
    }
}

pub async fn get_record(record: &DNSRecord, auth: &Auth) -> Vec<DNSRecord> {
    let api: Api = Api::Get(record.clone());

    let header = auth.as_header();

    let client = reqwest::Client::new();

    let req = client
        .get(api.to_string())
        .header("accept", "application/json")
        .header("authorization", &header);

    let response = req.send().await.expect("Error listing records.");
    if response.status().is_success() {
        let data = &response.text().await.expect("Error reading response text");
        let record: Vec<DNSRecord> = serde_json::from_str(data).expect("Error parsing response");
        return record;
    } else {
        return vec![];
    }
}

pub async fn list_records(domain: &str, auth: &Auth) -> () {
    let api: Api = Api::List(domain.to_string());

    let header = auth.as_header();

    let client = reqwest::Client::new();

    let req = client
        .get(api.to_string())
        .header("accept", "application/json")
        .header("authorization", &header);

    let response = req.send().await.expect("Error listing records.");
    if response.status().is_success() {
        let body = response.text().await.expect("Error reading response text");
        let records: Vec<DNSRecord> = serde_json::from_str(&body).unwrap();
        // .expect("Error parsing records into object");
        println!("{:<5} {:<25} {:<30}", "Type", "Name", "Value"); // Header
        for record in records {
            println!(
                "{:<5} {:<25} {:<30}",
                record.record_type, record.name, record.data
            );
        }
    } else {
        println!("Request failed with status: {}", response.status());
    }
}
