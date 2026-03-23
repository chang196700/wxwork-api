/// 集成测试：WxWorkClient 基础 HTTP 功能（代理配置、client 构建、check_base）
use wxwork_api::{WxWorkClient, WxWorkConfig, ProxyConfig, WxWorkError};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};

mod common;

// ── client 构建 ────────────────────────────────────────────────────────────

#[test]
fn test_client_build_no_proxy() {
    let cfg = WxWorkConfig::new("c", "s");
    assert!(WxWorkClient::new(cfg).is_ok());
}

#[test]
fn test_client_build_http_proxy() {
    let cfg = WxWorkConfig::new("c", "s")
        .with_proxy(ProxyConfig::http("http://127.0.0.1:8080"));
    assert!(WxWorkClient::new(cfg).is_ok());
}

#[test]
fn test_client_build_https_proxy() {
    let cfg = WxWorkConfig::new("c", "s")
        .with_proxy(ProxyConfig::https("https://127.0.0.1:8080"));
    assert!(WxWorkClient::new(cfg).is_ok());
}

#[test]
fn test_client_build_socks5_proxy() {
    let cfg = WxWorkConfig::new("c", "s")
        .with_proxy(ProxyConfig::socks5("socks5://127.0.0.1:1080"));
    assert!(WxWorkClient::new(cfg).is_ok());
}

#[test]
fn test_client_build_proxy_with_auth() {
    let cfg = WxWorkConfig::new("c", "s")
        .with_proxy(ProxyConfig::with_auth("socks5://127.0.0.1:1080", "user", "pass"));
    assert!(WxWorkClient::new(cfg).is_ok());
}

#[test]
fn test_client_build_invalid_proxy_url() {
    let cfg = WxWorkConfig::new("c", "s")
        .with_proxy(ProxyConfig::Http("not a url !!!".to_string()));
    assert!(WxWorkClient::new(cfg).is_err());
}

// ── check_response / check_base ──────────────────────────────────────────

#[test]
fn test_check_response_ok() {
    assert!(WxWorkClient::check_response(0, "ok").is_ok());
}

#[test]
fn test_check_response_err() {
    let result = WxWorkClient::check_response(40001, "invalid credential");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, WxWorkError::ApiError { errcode: 40001, .. }));
}

// ── access_token 获取 ────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_access_token_ok() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/gettoken"))
        .and(query_param("corpid", "corp_abc"))
        .and(query_param("corpsecret", "secret_xyz"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "access_token": "TOKEN_ABC",
            "expires_in": 7200
        })))
        .mount(&server)
        .await;

    let cfg = WxWorkConfig::new("corp_abc", "secret_xyz")
        .with_base_url(server.uri());
    let client = WxWorkClient::new(cfg).unwrap();

    let token = client.access_token().await.unwrap();
    assert_eq!(token, "TOKEN_ABC");
}

#[tokio::test]
async fn test_get_access_token_cached() {
    let server = MockServer::start().await;

    // Mount only once — second call must come from cache
    Mock::given(method("GET"))
        .and(path("/cgi-bin/gettoken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "access_token": "CACHED_TOKEN",
            "expires_in": 7200
        })))
        .expect(1)      // must be called exactly once
        .mount(&server)
        .await;

    let cfg = WxWorkConfig::new("corp_cache", "secret_cache")
        .with_base_url(server.uri());
    let client = WxWorkClient::new(cfg).unwrap();

    let t1 = client.access_token().await.unwrap();
    let t2 = client.access_token().await.unwrap();
    assert_eq!(t1, t2);
    // wiremock verifies the expect(1) assertion on drop
}

#[tokio::test]
async fn test_get_access_token_api_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/gettoken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 40013,
            "errmsg": "invalid corpid"
        })))
        .mount(&server)
        .await;

    let cfg = WxWorkConfig::new("bad_corp", "bad_secret")
        .with_base_url(server.uri());
    let client = WxWorkClient::new(cfg).unwrap();

    let result = client.access_token().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), WxWorkError::TokenError(_)));
}
