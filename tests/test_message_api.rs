/// 集成测试：消息发送 API
use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use wxwork_api::api::message::send::SendMessageRequest;

mod common;
use common::start_mock_server;

#[tokio::test]
async fn test_send_text_message_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/message/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "msgid": "MSG001",
            "invaliduser": "",
            "invalidparty": "",
            "invalidtag": ""
        })))
        .mount(&server)
        .await;

    let req = SendMessageRequest::text(1, "@all", "Hello!");
    let resp = client.message().send().send(&req).await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.msgid.unwrap(), "MSG001");
}

#[tokio::test]
async fn test_send_markdown_message_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/message/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "msgid": "MSG002"
        })))
        .mount(&server)
        .await;

    let req = SendMessageRequest::markdown(1, "user1", "# 标题\n内容");
    let resp = client.message().send().send(&req).await.unwrap();
    assert_eq!(resp.errcode, 0);
}

#[tokio::test]
async fn test_send_message_invalid_user() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/message/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "msgid": "MSG003",
            "invaliduser": "ghost_user"
        })))
        .mount(&server)
        .await;

    let req = SendMessageRequest::text(1, "ghost_user|real_user", "Hi");
    let resp = client.message().send().send(&req).await.unwrap();
    // API returns 0 even when some users are invalid; caller checks invaliduser
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.invaliduser.unwrap(), "ghost_user");
}

#[tokio::test]
async fn test_recall_message_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/message/recall"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    assert!(client.message().send().recall("MSG001").await.is_ok());
}

#[tokio::test]
async fn test_recall_message_not_found() {
    use wxwork_api::WxWorkError;
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/message/recall"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 45028,
            "errmsg": "msg not found"
        })))
        .mount(&server)
        .await;

    let result = client.message().send().recall("NONEXISTENT").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), WxWorkError::ApiError { errcode: 45028, .. }));
}
