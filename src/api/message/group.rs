use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 应用发送消息到群聊会话 API
pub struct GroupMessageApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> GroupMessageApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 创建群聊会话 POST /cgi-bin/appchat/create
    pub async fn create_chat(&self, req: &CreateChatRequest) -> Result<CreateChatResponse> {
        self.client.post("/cgi-bin/appchat/create", req).await
    }

    /// 修改群聊会话 POST /cgi-bin/appchat/update
    pub async fn update_chat(&self, req: &UpdateChatRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/appchat/update", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 获取群聊会话 GET /cgi-bin/appchat/get
    pub async fn get_chat(&self, chatid: &str) -> Result<GetChatResponse> {
        self.client
            .get("/cgi-bin/appchat/get", &[("chatid", chatid)])
            .await
    }

    /// 应用推送消息 POST /cgi-bin/appchat/send
    pub async fn send(&self, req: &ChatMessageRequest) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/appchat/send", req).await
    }
}

// ============ Request types ============

#[derive(Debug, Serialize)]
pub struct CreateChatRequest {
    pub name: String,
    pub owner: String,
    pub userlist: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatid: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateChatRequest {
    pub chatid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub add_user_list: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub del_user_list: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatMessageRequest {
    pub chatid: String,
    pub msgtype: String,
    #[serde(flatten)]
    pub content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<u8>,
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct CreateChatResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub chatid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetChatResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub chat_info: Option<ChatInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ChatInfo {
    pub chatid: String,
    pub name: Option<String>,
    pub owner: Option<String>,
    #[serde(default)]
    pub userlist: Vec<String>,
}
