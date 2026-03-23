#![allow(dead_code)]
/// 集成测试共用工具：启动 wiremock 服务器，构建指向该服务器的 WxWorkClient
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};
use wxwork_api::{WxWorkClient, WxWorkConfig};
/// 服务器已预置一个 /cgi-bin/gettoken 的成功响应，方便各测试复用
pub async fn start_mock_server() -> (MockServer, WxWorkClient) {
    let server = MockServer::start().await;

    // 预置 token 端点
    Mock::given(method("GET"))
        .and(path("/cgi-bin/gettoken"))
        .and(query_param("corpid", "test_corp"))
        .and(query_param("corpsecret", "test_secret"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "errcode": 0,
                "errmsg": "ok",
                "access_token": "MOCK_TOKEN",
                "expires_in": 7200
            }))
        )
        .mount(&server)
        .await;

    let cfg = WxWorkConfig::new("test_corp", "test_secret")
        .with_base_url(server.uri());

    let client = WxWorkClient::new(cfg).expect("failed to build client");
    (server, client)
}

/// 构建 token 端点响应 JSON
pub fn token_ok() -> serde_json::Value {
    serde_json::json!({
        "errcode": 0,
        "errmsg": "ok",
        "access_token": "MOCK_TOKEN",
        "expires_in": 7200
    })
}
