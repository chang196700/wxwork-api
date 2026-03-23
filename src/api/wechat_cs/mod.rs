use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 连接微信 - 微信客服 API
pub struct WechatCsApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> WechatCsApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    // ====== 客服账号管理 ======

    /// 添加客服账号 POST /cgi-bin/kf/account/add
    pub async fn add_account(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/account/add", req).await
    }

    /// 删除客服账号 POST /cgi-bin/kf/account/del
    pub async fn del_account(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/kf/account/del", req).await
    }

    /// 修改客服账号 POST /cgi-bin/kf/account/update
    pub async fn update_account(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/kf/account/update", req).await
    }

    /// 获取客服账号列表 GET /cgi-bin/kf/account/list
    pub async fn list_accounts(&self) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/kf/account/list", &[]).await
    }

    // ====== 接待人员管理 ======

    /// 添加接待人员 POST /cgi-bin/kf/servicer/add
    pub async fn add_servicer(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/servicer/add", req).await
    }

    /// 删除接待人员 POST /cgi-bin/kf/servicer/del
    pub async fn del_servicer(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/servicer/del", req).await
    }

    /// 获取接待人员列表 GET /cgi-bin/kf/servicer/list
    pub async fn list_servicers(&self, open_kfid: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/kf/servicer/list", &[("open_kfid", open_kfid)]).await
    }

    // ====== 会话分配与消息收发 ======

    /// 获取会话状态及接待人员 POST /cgi-bin/kf/service_state/get
    pub async fn get_service_state(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/service_state/get", req).await
    }

    /// 变更会话状态 POST /cgi-bin/kf/service_state/trans
    pub async fn trans_service_state(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/service_state/trans", req).await
    }

    /// 读取消息 POST /cgi-bin/kf/sync_msg
    pub async fn sync_msg(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/sync_msg", req).await
    }

    /// 发送消息 POST /cgi-bin/kf/send_msg
    pub async fn send_msg(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/send_msg", req).await
    }

    /// 发送欢迎语等事件响应消息 POST /cgi-bin/kf/send_msg_on_event
    pub async fn send_msg_on_event(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/send_msg_on_event", req).await
    }

    // ====== 统计管理 ======

    /// 获取「客服」账号统计数据 POST /cgi-bin/kf/get_corp_statistic
    pub async fn get_corp_statistic(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/get_corp_statistic", req).await
    }

    /// 获取「接待人员」统计数据 POST /cgi-bin/kf/get_servicer_statistic
    pub async fn get_servicer_statistic(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/kf/get_servicer_statistic", req).await
    }

    /// 通用扩展调用
    pub async fn call_post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }
}
