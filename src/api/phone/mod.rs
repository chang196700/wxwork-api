use crate::client::WxWorkClient;
use crate::error::Result;

/// 办公 - 公费电话 API
pub struct PhoneApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> PhoneApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取公费电话拨打记录 POST /cgi-bin/dial/get_dial_record
    pub async fn get_dial_record(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/dial/get_dial_record", req).await
    }
}
