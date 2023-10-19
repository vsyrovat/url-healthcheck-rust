mod args;

use args::Args;
use clap::Parser;
use reqwest::Client;
use tokio::time::{sleep, Duration};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    match Url::parse(&args.url) {
        Ok(_url) => loop {
            match client.get(&args.url).send().await {
                Ok(response) => {
                    if response.status() == 200 {
                        println!("Checking '{}'. Result: OK(200)", &args.url);
                    } else {
                        println!(
                            "Checking '{}'. Result: ERR({})",
                            response.url(),
                            response.status().as_u16()
                        );
                    }
                }
                Err(e) => {
                    println!("Checking '{}'. Unexpected error: {}", &args.url, e);
                }
            }
            sleep(Duration::from_secs(args.delay as u64)).await
        },
        Err(_) => {
            println!("URL parsing error");
        }
    }

    Ok(())
}
