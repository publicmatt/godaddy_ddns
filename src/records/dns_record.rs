use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    SOA,
    SRV,
    TXT,
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            RecordType::A => "A",
            RecordType::AAAA => "AAAA",
            RecordType::CNAME => "CNAME",
            RecordType::MX => "MX",
            RecordType::NS => "NS",
            RecordType::SOA => "SOA",
            RecordType::SRV => "SRV",
            RecordType::TXT => "TXT",
        };
        f.write_fmt(format_args!(
            "{:<width$}",
            text,
            width = f.width().unwrap_or(0)
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DNSRecord {
    #[serde(default)]
    #[serde(skip_serializing)]
    pub domain: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    #[serde(rename = "type")]
    pub record_type: RecordType,

    #[serde(default)]
    pub data: String,

    #[serde(default = "default_ttl")]
    pub ttl: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub port: Option<u16>, // SRV Only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub priority: Option<u32>, // MX and SRV only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub protocol: Option<String>, // SRV only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub service: Option<String>, // SRV only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub weight: Option<u32>, // SRV only.
}

impl Default for RecordType {
    fn default() -> Self {
        RecordType::A
    }
}

fn default_ttl() -> u32 {
    600
}
