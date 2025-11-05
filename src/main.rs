use clap::Parser;
use rustid::config::Config;
use rustid::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    serve(config).await?;

    Ok(())
}
