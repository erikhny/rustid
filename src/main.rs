use clap::Parser;
use rustid::config::Config;
use rustid::http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    http::serve(config).await?;

    Ok(())
}
