use tokio::net::TcpListener;
use rustid::config::{get_configuration};
use rustid::{create_router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = get_configuration().expect("failed to get configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let router = create_router(configuration).await?;
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
