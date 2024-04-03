extern crate dotenv;
use dotenv::dotenv;
use std::env;

use std::error::Error;

use log::LevelFilter;
use simple_logger::SimpleLogger;

mod auth;
mod godaddy;
mod ip_handler;
pub mod records;
use crate::auth::Auth;
use records::dns_record::{DNSRecord, RecordType};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    author = "publicmatt",
    version = "1.0",
    about = "add/update/delete godaddy dns records"
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    #[clap(about = "list dns record names/types")]
    List(ListCmd),

    #[clap(about = "delete dns record name/type")]
    Delete(DeleteCmd),

    #[clap(about = "add/update dns record name/type")]
    Update(UpdateCmd),
}

#[derive(Parser, Debug)]
struct UpdateCmd {
    #[clap(long)]
    domain: String,

    /// Subdomain
    #[clap(long)]
    name: String,

    #[clap(long)]
    ttl: Option<u32>,

    #[clap(long, default_value_t = RecordType::A)]
    record_type: RecordType,

    /// IP Address or data for the record
    #[clap(long)]
    data: String,
}

#[derive(Parser, Debug)]
struct DeleteCmd {
    #[clap(long)]
    domain: String,

    /// Subdomain
    #[clap(long)]
    name: String,

    #[clap(long, default_value_t = RecordType::A)]
    record_type: RecordType,
}

#[derive(Parser, Debug)]
struct ListCmd {
    #[clap(long)]
    domain: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_colors(true)
        .with_utc_timestamps()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    dotenv().ok();

    let auth: Auth = Auth {
        key: env::var("GODADDY_KEY").expect("You need to set GODADDY_KEY env variable first."),
        secret: env::var("GODADDY_SECRET")
            .expect("You need to set GODADDY_SECRET env variable first."),
    };

    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Update(args) => {
            let record = DNSRecord {
                domain: args.domain,
                name: args.name,
                record_type: args.record_type,
                data: args.data,
                ttl: 600,
                ..Default::default()
            };
            records::update_record(record, &auth).await;
            return Ok(());
        }
        SubCommands::Delete(args) => {
            let record = DNSRecord {
                domain: args.domain,
                name: args.name,
                record_type: args.record_type,
                ..Default::default()
            };
            records::delete_record(record, &auth).await;
            return Ok(());
        }
        SubCommands::List(args) => {
            let domain = args.domain;
            records::list_records(&domain, &auth).await;
            return Ok(());
        }
    }
}
