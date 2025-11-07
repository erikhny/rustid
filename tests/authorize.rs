use axum::response::Redirect;
use tokio::net::TcpListener;
use rustid::config::get_configuration;

#[tokio::test]
async fn test_authorize() {
    let addr = spawn_app();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let response = client
        .get(format!("{}/oauth/authorize", addr))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_redirection());
}

fn spawn_app() -> String {
    let port = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
        listener.local_addr().unwrap().port()
    };

    let _ = tokio::spawn(async move {
        let configuration = get_configuration().expect("could not get configuration");
        let router = rustid::create_router(configuration).await.expect("failed to create router");
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .expect("failed to set listener");

        axum::serve(listener, router).await.expect("failed to serve app");
    });

    format!("http://127.0.0.1:{}", port)
}