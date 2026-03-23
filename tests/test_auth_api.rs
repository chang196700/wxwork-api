/// 集成测试：Auth API（获取 access_token、IP 列表）
use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

mod common;
use common::start_mock_server;

#[tokio::test]
async fn test_auth_get_access_token() {
    let (server, client) = start_mock_server().await;

    // Token endpoint already mounted by start_mock_server.
    // The auth API wraps the same token mechanism.
    Mock::given(method("GET"))
        .and(path("/cgi-bin/gettoken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "access_token": "AUTH_TOKEN",
            "expires_in": 7200
        })))
        .mount(&server)
        .await;

    // The token manager is shared; first call fills the cache.
    let token = client.access_token().await.unwrap();
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_auth_get_api_domain_ip() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/get_api_domain_ip"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "ip_list": ["101.91.62.0/26", "182.254.11.176/28"]
        })))
        .mount(&server)
        .await;

    let resp = client.auth().get_api_domain_ip().await.unwrap();
    assert_eq!(resp.errcode, 0);
    let ips = resp.ip_list.unwrap_or_default();
    assert_eq!(ips.len(), 2);
    assert!(ips.contains(&"101.91.62.0/26".to_string()));
}

#[tokio::test]
async fn test_auth_get_callback_ip() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/getcallbackip"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "ip_list": ["140.207.54.0/24"]
        })))
        .mount(&server)
        .await;

    let resp = client.auth().get_callback_ip().await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert!(!resp.ip_list.unwrap_or_default().is_empty());
}
