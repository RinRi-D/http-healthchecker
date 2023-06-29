use tokio::time;
use std::time::Duration;
use std::cmp::Ordering;
use chrono::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// interval between requests in ms
    #[arg(short, long, default_value_t = 1000)]
    interval: u64,

    /// request timeout in ms
    #[arg(short, long, default_value_t = 1000)]
    timeout: u64,

    uri: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let uri = args.uri;

    // interval between requests
    let interval_ms = args.interval;
    let mut interval = time::interval(Duration::from_millis(interval_ms));

    // request timeout
    let timeout_ms = match args.timeout.cmp(&interval_ms) {
        Ordering::Greater => {
            println!("Warning: timeout can not be greater than interval. Defaulting to {}ms", 
                interval_ms);
            interval_ms
        }
        _ => args.timeout,
    };

    println!("Healthchecking {} every {}ms, with timeout of {}ms",
        uri, interval_ms, timeout_ms);

    loop {
        interval.tick().await;
        print!("{}: ", Utc::now());

        match time::timeout(Duration::from_millis(timeout_ms), reqwest::get(&uri)).await {
            Ok(req) => match req {
                Ok(res) => {
                    println!("Status: {}", res.status());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            },
            Err(_) => {
                println!("Error: timeout of {}ms exceeded", timeout_ms);
            }
        };
    }
}
