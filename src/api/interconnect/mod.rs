use crate::client::WxWorkClient;
use crate::error::Result;

/// 企业互联 API
pub struct InterconnectApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> InterconnectApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取下级/下游企业的 access_token POST /cgi-bin/corp/get_token
    pub async fn get_corp_token(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corp/get_token", req).await
    }

    /// 获取下级/下游企业小程序 session POST /cgi-bin/corp/miniprogram/transfer_session
    pub async fn transfer_session(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corp/miniprogram/transfer_session", req).await
    }

    /// 获取应用共享信息 POST /cgi-bin/corp/get_agent_info
    pub async fn get_agent_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corp/get_agent_info", req).await
    }
}
