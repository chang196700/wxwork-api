use serde::{Deserialize, Serialize};
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 日程 API
pub struct CalendarApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> CalendarApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 创建日历 POST /cgi-bin/oa/calendar/add
    pub async fn add_calendar(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/calendar/add", req).await
    }

    /// 更新日历 POST /cgi-bin/oa/calendar/update
    pub async fn update_calendar(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/oa/calendar/update", req).await
    }

    /// 删除日历 POST /cgi-bin/oa/calendar/del
    pub async fn del_calendar(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/oa/calendar/del", req).await
    }

    /// 获取日历详情 POST /cgi-bin/oa/calendar/get
    pub async fn get_calendar(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/calendar/get", req).await
    }

    /// 创建日程 POST /cgi-bin/oa/schedule/add
    pub async fn add_schedule(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/schedule/add", req).await
    }

    /// 更新日程 POST /cgi-bin/oa/schedule/update
    pub async fn update_schedule(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/oa/schedule/update", req).await
    }

    /// 删除日程 POST /cgi-bin/oa/schedule/del
    pub async fn del_schedule(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/oa/schedule/del", req).await
    }

    /// 获取日程详情 POST /cgi-bin/oa/schedule/get
    pub async fn get_schedule(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/schedule/get", req).await
    }

    /// 获取日程列表 POST /cgi-bin/oa/schedule/get_by_calendar
    pub async fn get_schedule_list_by_calendar(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/schedule/get_by_calendar", req).await
    }
}
