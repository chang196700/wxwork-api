use serde::{Deserialize, Serialize};
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 会议 API
///
/// 覆盖以下子模块：
/// - 预约会议（基础/高级）
/// - 会中控制
/// - 录制管理
/// - 会议统计
/// - 会议布局和背景
/// - 网络研讨会（Webinar）
/// - 电话入会（PSTN）
/// - 会议室连接器（MRA）
/// - Rooms 会议室
/// - 高级功能账号
/// - 回调通知（接收端，无需调用）
pub struct MeetingApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> MeetingApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    // ====== 预约会议基础管理 ======

    /// 创建预约会议 POST /cgi-bin/meeting/create
    pub async fn create(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/meeting/create", req).await
    }

    /// 修改预约会议 POST /cgi-bin/meeting/update
    pub async fn update(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/update", req).await
    }

    /// 取消预约会议 POST /cgi-bin/meeting/cancel
    pub async fn cancel(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/cancel", req).await
    }

    /// 获取预约会议详情 POST /cgi-bin/meeting/get_info
    pub async fn get_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/meeting/get_info", req).await
    }

    /// 获取成员预约会议列表 POST /cgi-bin/meeting/get_user_meetingid
    pub async fn get_user_meeting_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/meeting/get_user_meetingid", req).await
    }

    /// 获取用户参与会议列表 POST /cgi-bin/meeting/get_user_meetingid
    pub async fn get_guests(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/meeting/guests/get", req).await
    }

    // ====== 会中控制 ======

    /// 踢出参会成员 POST /cgi-bin/meeting/ctrl/kick_users
    pub async fn kick_users(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/ctrl/kick_users", req).await
    }

    /// 修改成员状态 POST /cgi-bin/meeting/ctrl/modify_user_state
    pub async fn modify_user_state(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/ctrl/modify_user_state", req).await
    }

    /// 结束会议 POST /cgi-bin/meeting/ctrl/end
    pub async fn end_meeting(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/ctrl/end", req).await
    }

    // ====== 录制管理 ======

    /// 获取会议录制列表 POST /cgi-bin/meeting/record/get_meeting_record_info
    pub async fn get_meeting_record_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/meeting/record/get_meeting_record_info", req).await
    }

    /// 删除会议录制 POST /cgi-bin/meeting/record/delete
    pub async fn delete_record(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/record/delete", req).await
    }

    // ====== 会议统计 ======

    /// 获取会议统计信息 POST /cgi-bin/meeting/stats/get_meeting_info
    pub async fn get_stats(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/meeting/stats/get_meeting_info", req).await
    }

    // ====== 高级功能账号管理 ======

    /// 分配高级功能账号 POST /cgi-bin/meeting/advanced_account/assign
    pub async fn assign_advanced_account(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/advanced_account/assign", req).await
    }

    /// 取消高级功能账号 POST /cgi-bin/meeting/advanced_account/cancel
    pub async fn cancel_advanced_account(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/meeting/advanced_account/cancel", req).await
    }

    /// 获取高级功能账号列表 POST /cgi-bin/meeting/advanced_account/list
    pub async fn list_advanced_accounts(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/meeting/advanced_account/list", req).await
    }

    // ====== 通用扩展接口 ======

    /// 调用任意会议子 API POST
    pub async fn call_post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    /// 调用任意会议子 API GET
    pub async fn call_get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }
}
