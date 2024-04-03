use serde::Deserialize;
use std::fmt;

use crate::dns::DNSRecord;

pub enum Api {
    Patch(DNSRecord),
    Delete(DNSRecord),
    Get(DNSRecord),
    List(String),
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
    pub fields: Vec<ResponseField>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseField {
    #[serde(default)]
    pub code: String,

    #[serde(default)]
    pub message: String,

    #[serde(default)]
    pub path: String,

    #[serde(default)]
    #[serde(rename = "pathRelated")]
    pub path_related: String,
}

impl fmt::Display for Api {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Api::Patch(record) => write!(
                f,
                "https://api.godaddy.com/v1/domains/{domain}/records",
                domain = record.domain,
            ),
            Api::Delete(record) => write!(
                f,
                "https://api.godaddy.com/v1/domains/{domain}/records/{record_type}/{name}",
                domain = record.domain,
                record_type = record.record_type,
                name = record.name
            ),
            Api::Get(record) => write!(
                f,
                "https://api.godaddy.com/v1/domains/{domain}/records/{record_type}/{name}",
                domain = record.domain,
                record_type = record.record_type,
                name = record.name
            ),
            Api::List(domain) => write!(
                f,
                "https://api.godaddy.com/v1/domains/{domain}/records",
                domain = domain,
            ),
        }
    }
}
