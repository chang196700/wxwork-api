use crate::client::WxWorkClient;
use crate::error::Result;

/// 数据与智能专区 API
pub struct DataIntelApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> DataIntelApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 数据接口 - 通用 POST
    pub async fn post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    /// 数据接口 - 通用 GET
    pub async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }

    /// 获取打卡数据 POST /cgi-bin/checkin/getcheckindata
    pub async fn get_checkin_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckindata", req).await
    }

    /// 获取打卡日报 POST /cgi-bin/checkin/getcheckin_daydata
    pub async fn get_checkin_day_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckin_daydata", req).await
    }

    /// 获取打卡月报 POST /cgi-bin/checkin/getcheckin_monthdata
    pub async fn get_checkin_month_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckin_monthdata", req).await
    }
}
