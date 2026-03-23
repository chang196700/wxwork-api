use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 连接微信 - 家校沟通 + 家校应用 API
pub struct SchoolApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> SchoolApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    // ====== 家校沟通基础接口 ======

    /// 获取「学校」access_token POST /cgi-bin/school/get_school_acess_token（文档拼写）
    pub async fn get_school_access_token(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/school/get_school_acess_token", req).await
    }

    /// 获取学生/家长详情 POST /cgi-bin/school/user/get_external_contact
    pub async fn get_external_contact(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/school/user/get_external_contact", req).await
    }

    /// 发送学校通知 POST /cgi-bin/message/send_school_notice
    pub async fn send_school_notice(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/message/send_school_notice", req).await
    }

    /// 通用扩展调用
    pub async fn call_post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    pub async fn call_get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }
}
