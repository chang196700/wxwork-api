use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 会议室 API
pub struct MeetingRoomApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> MeetingRoomApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 会议室管理 - 通用 POST
    pub async fn post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    /// 会议室管理 - 通用 GET
    pub async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }

    /// 获取会议室列表 POST /cgi-bin/oa/meetingroom/list
    pub async fn list_rooms(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/meetingroom/list", req).await
    }

    /// 预定会议室 POST /cgi-bin/oa/meetingroom/book
    pub async fn book_room(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/meetingroom/book", req).await
    }

    /// 取消预定 POST /cgi-bin/oa/meetingroom/cancel_book
    pub async fn cancel_book(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/oa/meetingroom/cancel_book", req).await
    }

    /// 查询会议室预定信息 POST /cgi-bin/oa/meetingroom/getbooklist
    pub async fn get_book_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/meetingroom/getbooklist", req).await
    }
}
