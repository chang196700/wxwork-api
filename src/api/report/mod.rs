use crate::client::WxWorkClient;
use crate::error::Result;

/// 办公 - 汇报 API
pub struct ReportApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> ReportApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取汇报记录列表 POST /cgi-bin/oa/journal/get_record_list
    pub async fn get_record_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/journal/get_record_list", req).await
    }

    /// 获取汇报记录详情 POST /cgi-bin/oa/journal/get_record_detail
    pub async fn get_record_detail(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/journal/get_record_detail", req).await
    }

    /// 获取汇报统计数据 POST /cgi-bin/oa/journal/get_statistic_data
    pub async fn get_statistic_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/journal/get_statistic_data", req).await
    }
}
