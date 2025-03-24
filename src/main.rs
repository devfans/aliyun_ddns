use std::{env, path::PathBuf};
use aliyun_dns::AliyunDns;
use clap::{command, Parser, Subcommand};
use lazy_static::lazy_static;

lazy_static! {
    static ref access_key_id: String = env::var("ALI_KEY").unwrap();
    static ref access_key_secret: String = env::var("ALI_SECRET").unwrap();
    static ref dns: AliyunDns = AliyunDns::new(access_key_id.to_string(), access_key_secret.to_string());
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        domain_name: String,
        #[arg(short, long)]
        sub_domain: String,
        #[arg(short, long)]
        record_type: String,
        #[arg(short, long)]
        value: String,
    },
    Update {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        sub_domain: String,
        #[arg(short, long)]
        record_type: String,
        #[arg(short, long)]
        value: String,
    },
    List {
        #[arg(short, long)]
        domain: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add { domain_name, sub_domain, record_type, value }) => {
            add_subdomain(domain_name, sub_domain, record_type, value).await;
        },
        Some(Commands::List { domain }) => {
            list_subdomains(&domain).await;
        },
        Some(Commands::Update { id, sub_domain, record_type, value }) => {
            update_subdomain(id, sub_domain, record_type, value).await;
        },
        None => { eprintln!("invalid sub command"); },
    }
}

async fn add_subdomain(domain_name: &str, sub_domain: &str, record_type: &str, record_value: &str) {
    match dns.add_domain_record(domain_name, sub_domain, record_type, record_value).await {
        Ok(res) => {
            eprintln!("{:?}", res);
        },
        Err(err) => { eprintln!("{}", err); }
    }
}

async fn update_subdomain(record_id: &str, sub_domain: &str, record_type: &str, value: &str) {
    match dns.update_domain_record(record_id, sub_domain, record_type, value).await {
        Ok(res) => {
            eprintln!("{:?}", res);
        },
        Err(err) => { eprintln!("{}", err); }
    }
}

async fn list_subdomains(domain_name: &str) {
    match dns.query_domain_records(domain_name).await {
        Ok(res) => {
            eprintln!("{:?}", res);
        },
        Err(err) => { eprintln!("{}", err); }
    }
}