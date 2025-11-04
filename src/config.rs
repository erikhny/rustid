#[derive(clap::Parser)]
pub struct Config {
    #[clap(long)]
    pub database_url: Option<String>,
    #[clap(long)]
    pub hmac_key: Option<String>
}