use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 人事助手 API
pub struct HrApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> HrApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取花名册字段信息 POST /cgi-bin/hr/get_columns
    pub async fn get_columns(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/hr/get_columns", req).await
    }

    /// 获取员工花名册信息 POST /cgi-bin/hr/get_member_detail
    pub async fn get_member_detail(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/hr/get_member_detail", req).await
    }

    /// 更新员工花名册信息 POST /cgi-bin/hr/update_member_detail
    pub async fn update_member_detail(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/hr/update_member_detail", req).await
    }
}
