use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 消息接收与发送 - 发送应用消息 API
pub struct MessageSendApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> MessageSendApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 发送应用消息 POST /cgi-bin/message/send
    pub async fn send(&self, req: &SendMessageRequest) -> Result<SendMessageResponse> {
        self.client.post("/cgi-bin/message/send", req).await
    }

    /// 撤回应用消息 POST /cgi-bin/message/recall
    pub async fn recall(&self, msgid: &str) -> Result<()> {
        let body = serde_json::json!({ "msgid": msgid });
        let resp: BaseResponse = self.client.post("/cgi-bin/message/recall", &body).await?;
        WxWorkClient::check_base(resp)
    }

    /// 更新模板卡片消息 POST /cgi-bin/message/update_template_card
    pub async fn update_template_card(
        &self,
        req: &UpdateTemplateCardRequest,
    ) -> Result<UpdateTemplateCardResponse> {
        self.client
            .post("/cgi-bin/message/update_template_card", req)
            .await
    }
}

// ============ Request types ============

/// 发送应用消息请求
#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    /// 接收成员，多个用 | 分隔，最多1000个，@all 表示全部
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,
    /// 接收部门 ID，多个用 | 分隔
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toparty: Option<String>,
    /// 接收标签 ID，多个用 | 分隔
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totag: Option<String>,
    /// 消息类型
    pub msgtype: String,
    /// 应用 ID
    pub agentid: i64,
    /// 消息内容（根据 msgtype 不同类型）
    #[serde(flatten)]
    pub content: MessageContent,
    /// 是否保密，0=否，1=是（显示水印），默认0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<u8>,
    /// 是否开启 ID 转译，0/1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<u8>,
    /// 是否开启重复消息检查
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_check: Option<u8>,
    /// 重复消息检查时间间隔（秒），默认1800
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check_interval: Option<u32>,
}

impl SendMessageRequest {
    /// 发送文本消息快捷构造
    pub fn text(agentid: i64, to_user: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            touser: Some(to_user.into()),
            toparty: None,
            totag: None,
            msgtype: "text".to_string(),
            agentid,
            content: MessageContent::Text(TextMessage {
                text: TextContent { content: content.into() },
            }),
            safe: None,
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
        }
    }

    /// 发送 markdown 消息快捷构造
    pub fn markdown(agentid: i64, to_user: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            touser: Some(to_user.into()),
            toparty: None,
            totag: None,
            msgtype: "markdown".to_string(),
            agentid,
            content: MessageContent::Markdown(MarkdownMessage {
                markdown: MarkdownContent { content: content.into() },
            }),
            safe: None,
            enable_id_trans: None,
            enable_duplicate_check: None,
            duplicate_check_interval: None,
        }
    }
}

/// 消息内容（按消息类型区分）
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(TextMessage),
    Image(ImageMessageWrapper),
    Voice(VoiceMessageWrapper),
    Video(VideoMessageWrapper),
    File(FileMessageWrapper),
    Textcard(TextcardMessageWrapper),
    News(NewsMessageWrapper),
    Mpnews(MpnewsMessageWrapper),
    Markdown(MarkdownMessage),
    MiniProgramNotice(MiniProgramNoticeWrapper),
    TemplateCard(TemplateCardWrapper),
}

#[derive(Debug, Serialize)]
pub struct TextMessage {
    pub text: TextContent,
}

#[derive(Debug, Serialize)]
pub struct TextContent {
    pub content: String,
}

// ============ Message content structures ============

#[derive(Debug, Serialize)]
pub struct MarkdownMessage {
    pub markdown: MarkdownContent,
}

#[derive(Debug, Serialize)]
pub struct MarkdownContent {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ImageMessageWrapper {
    pub image: MediaIdContent,
}

#[derive(Debug, Serialize)]
pub struct VoiceMessageWrapper {
    pub voice: MediaIdContent,
}

#[derive(Debug, Serialize)]
pub struct VideoMessageWrapper {
    pub video: VideoContent,
}

#[derive(Debug, Serialize)]
pub struct FileMessageWrapper {
    pub file: MediaIdContent,
}

#[derive(Debug, Serialize)]
pub struct TextcardMessageWrapper {
    pub textcard: TextcardContent,
}

#[derive(Debug, Serialize)]
pub struct NewsMessageWrapper {
    pub news: NewsContent,
}

#[derive(Debug, Serialize)]
pub struct MpnewsMessageWrapper {
    pub mpnews: MpnewsContent,
}

#[derive(Debug, Serialize)]
pub struct MiniProgramNoticeWrapper {
    pub miniprogram_notice: MiniProgramNoticeContent,
}

#[derive(Debug, Serialize)]
pub struct TemplateCardWrapper {
    pub template_card: Value,
}

#[derive(Debug, Serialize)]
pub struct MediaIdContent {
    pub media_id: String,
}

#[derive(Debug, Serialize)]
pub struct VideoContent {
    pub media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TextcardContent {
    pub title: String,
    pub description: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btntxt: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NewsContent {
    pub articles: Vec<NewsArticle>,
}

#[derive(Debug, Serialize)]
pub struct NewsArticle {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MpnewsContent {
    pub articles: Vec<MpnewsArticle>,
}

#[derive(Debug, Serialize)]
pub struct MpnewsArticle {
    pub title: String,
    pub thumb_media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_source_url: Option<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cover_pic: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct MiniProgramNoticeContent {
    pub appid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasis_first_item: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub content_item: Vec<MiniProgramNoticeItem>,
}

#[derive(Debug, Serialize)]
pub struct MiniProgramNoticeItem {
    pub key: String,
    pub value: String,
}

/// 更新模板卡片请求
#[derive(Debug, Serialize)]
pub struct UpdateTemplateCardRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toparty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totag: Option<String>,
    pub agentid: i64,
    pub response_code: String,
    pub button: Option<TemplateCardButton>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateCardButton {
    pub replace_name: String,
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub invaliduser: Option<String>,
    pub invalidparty: Option<String>,
    pub invalidtag: Option<String>,
    pub unlicenseduser: Option<String>,
    pub msgid: Option<String>,
    pub response_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateCardResponse {
    pub errcode: i32,
    pub errmsg: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_message_serialization() {
        let req = SendMessageRequest::text(1, "@all", "Hello World");
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["msgtype"], "text");
        assert_eq!(json["agentid"], 1);
        assert_eq!(json["touser"], "@all");
        assert_eq!(json["text"]["content"], "Hello World");
    }

    #[test]
    fn test_markdown_message_serialization() {
        let req = SendMessageRequest::markdown(42, "user1|user2", "# Title\ncontent");
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["msgtype"], "markdown");
        assert_eq!(json["agentid"], 42);
        assert_eq!(json["touser"], "user1|user2");
        assert_eq!(json["markdown"]["content"], "# Title\ncontent");
    }

    #[test]
    fn test_text_message_skips_null_recipients() {
        let req = SendMessageRequest::text(1, "@all", "Hi");
        let json = serde_json::to_value(&req).unwrap();
        // toparty and totag should be absent (skip_serializing_if = None)
        assert!(json.get("toparty").is_none() || json["toparty"].is_null());
        assert!(json.get("totag").is_none() || json["totag"].is_null());
    }

    #[test]
    fn test_text_message_safe_flag() {
        let mut req = SendMessageRequest::text(1, "@all", "secret");
        req.safe = Some(1);
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["safe"], 1);
    }

    #[test]
    fn test_send_message_response_deserialization() {
        let json = r#"{
            "errcode": 0,
            "errmsg": "ok",
            "msgid": "MSG_ID_001",
            "invaliduser": "",
            "invalidparty": "",
            "invalidtag": ""
        }"#;
        let r: SendMessageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.errcode, 0);
        assert_eq!(r.msgid.unwrap(), "MSG_ID_001");
    }
}
