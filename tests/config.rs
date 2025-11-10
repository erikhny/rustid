use rustid::config::get_configuration;

#[tokio::test]
async fn test_read_config() {
    let settings = get_configuration().unwrap();

    assert_eq!(settings.clients.len(), 2);
    assert_ne!(settings.clients.get("test-public-client"), None);
}