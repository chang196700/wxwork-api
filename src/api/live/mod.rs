use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 直播 API
pub struct LiveApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> LiveApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 创建直播 POST /cgi-bin/living/create
    pub async fn create(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/living/create", req).await
    }

    /// 修改直播 POST /cgi-bin/living/modify
    pub async fn modify(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/living/modify", req).await
    }

    /// 取消直播 POST /cgi-bin/living/cancel
    pub async fn cancel(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/living/cancel", req).await
    }

    /// 删除直播回放 POST /cgi-bin/living/delete_replay_data
    pub async fn delete_replay(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/living/delete_replay_data", req).await
    }

    /// 获取直播详情 GET /cgi-bin/living/get_living_info
    pub async fn get_living_info(&self, livingid: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/living/get_living_info", &[("livingid", livingid)]).await
    }

    /// 获取成员直播 ID 列表 GET /cgi-bin/living/get_user_all_livingid
    pub async fn get_user_all_livingid(&self, userid: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/living/get_user_all_livingid", &[("userid", userid)]).await
    }

    /// 获取直播观看明细 POST /cgi-bin/living/get_watch_stat
    pub async fn get_watch_stat(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/living/get_watch_stat", req).await
    }

    /// 获取跳转小程序商城的直播观看 URL POST /cgi-bin/living/get_living_share_info
    pub async fn get_living_share_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/living/get_living_share_info", req).await
    }
}
