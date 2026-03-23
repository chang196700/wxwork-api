use crate::client::WxWorkClient;
use crate::error::Result;

/// 连接微信 - 会话内容存档 API
///
/// 注意：完整的会话内容存档需要配合企业微信提供的 C SDK（libWeWorkFinanceSdk），
/// 此处提供 HTTP 接口部分（获取存档开启成员列表等）。
pub struct ChatArchiveApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> ChatArchiveApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取会话内容存档开启成员列表 GET /cgi-bin/msgaudit/get_permit_user_list
    pub async fn get_permit_user_list(&self, msg_type: Option<u8>) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, &str)> = vec![];
        let type_s;
        if let Some(t) = msg_type {
            type_s = t.to_string();
            query.push(("type", &type_s));
        }
        self.client.get("/cgi-bin/msgaudit/get_permit_user_list", &query).await
    }

    /// 获取会话同意情况 POST /cgi-bin/msgaudit/check_single_agree
    pub async fn check_single_agree(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/msgaudit/check_single_agree", req).await
    }

    /// 获取会话同意情况（群） POST /cgi-bin/msgaudit/check_room_agree
    pub async fn check_room_agree(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/msgaudit/check_room_agree", req).await
    }

    /// 获取会话内容存档内部群信息 POST /cgi-bin/msgaudit/groupchat/get
    pub async fn get_groupchat(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/msgaudit/groupchat/get", req).await
    }

    /// 注意：拉取消息记录（GetChatData）需要企业微信 C SDK，不在此 HTTP 客户端范围内
    pub fn note_get_chat_data() {
        // 拉取消息记录（GetChatData）需要企业微信提供的 C SDK（libWeWorkFinanceSdk），不在此 HTTP 客户端范围内
    }
}
