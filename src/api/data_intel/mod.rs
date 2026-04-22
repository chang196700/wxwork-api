use crate::client::WxWorkClient;

/// 数据与智能专区 API（待实现）
#[allow(dead_code)]
pub struct DataIntelApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> DataIntelApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }
}
