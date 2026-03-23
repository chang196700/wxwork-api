use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 紧急通知应用 API
pub struct AlertApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> AlertApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取紧急通知成员列表 POST /cgi-bin/oa/emergency_contact/get
    pub async fn get_emergency_contact(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/emergency_contact/get", req).await
    }
}
