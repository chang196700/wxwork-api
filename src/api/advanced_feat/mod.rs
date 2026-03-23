use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 高级功能 API
pub struct AdvancedFeatApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> AdvancedFeatApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    pub async fn post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    pub async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }
}
