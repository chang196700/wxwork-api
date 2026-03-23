use serde::{Deserialize, Serialize};
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 打卡 API
pub struct CheckinApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> CheckinApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取企业所有打卡规则 POST /cgi-bin/checkin/getcorpcheckinoption
    pub async fn get_corp_checkin_option(&self) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcorpcheckinoption", &serde_json::json!({})).await
    }

    /// 获取员工打卡规则 POST /cgi-bin/checkin/getcheckinoption
    pub async fn get_checkin_option(&self, req: &GetCheckinOptionRequest) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckinoption", req).await
    }

    /// 获取打卡记录数据 POST /cgi-bin/checkin/getcheckindata
    pub async fn get_checkin_data(&self, req: &GetCheckinDataRequest) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckindata", req).await
    }

    /// 获取打卡日报数据 POST /cgi-bin/checkin/getcheckin_daydata
    pub async fn get_checkin_day_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckin_daydata", req).await
    }

    /// 获取打卡月报数据 POST /cgi-bin/checkin/getcheckin_monthdata
    pub async fn get_checkin_month_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckin_monthdata", req).await
    }

    /// 获取打卡人员排班信息 POST /cgi-bin/checkin/getcheckinschedulist
    pub async fn get_checkin_schedule_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/getcheckinschedulist", req).await
    }

    /// 为打卡人员排班 POST /cgi-bin/checkin/setcheckinschedulist
    pub async fn set_checkin_schedule_list(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/checkin/setcheckinschedulist", req).await
    }

    /// 为打卡人员补卡 POST /cgi-bin/checkin/addcheckinuserface
    pub async fn add_checkin_user_face(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/checkin/addcheckinuserface", req).await
    }

    /// 添加打卡记录 POST /cgi-bin/checkin/add_checkin_feedback
    pub async fn add_checkin_feedback(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/checkin/add_checkin_feedback", req).await
    }

    /// 录入打卡人员人脸信息 POST /cgi-bin/checkin/addcheckinuserface
    pub async fn record_user_face(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/checkin/addcheckinuserface", req).await
    }

    /// 获取设备打卡数据 POST /cgi-bin/checkin/get_hardware_checkin_data
    pub async fn get_hardware_checkin_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/checkin/get_hardware_checkin_data", req).await
    }

    /// 管理打卡规则 POST /cgi-bin/checkin/create_checkin_option
    pub async fn manage_checkin_option(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }
}

#[derive(Debug, Serialize)]
pub struct GetCheckinOptionRequest {
    pub datetime: i64,
    pub useridlist: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct GetCheckinDataRequest {
    pub opencheckindatatype: u8,
    pub starttime: i64,
    pub endtime: i64,
    pub useridlist: Vec<String>,
}
