use crate::client::WxWorkClient;

/// 办公 - 高级功能 API（待实现）
#[allow(dead_code)]
pub struct AdvancedFeatApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> AdvancedFeatApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }
}
