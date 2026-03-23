use serde::Deserialize;
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 上下游 API
pub struct UpstreamApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> UpstreamApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取下级/下游企业的 access_token POST /cgi-bin/corpgroup/corp/gettoken
    pub async fn get_corp_token(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corpgroup/corp/gettoken", req).await
    }

    /// 获取下级/下游企业小程序 session POST /cgi-bin/miniprogram/transfer_session
    pub async fn transfer_session(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/miniprogram/transfer_session", req).await
    }

    /// 获取应用共享信息 POST /cgi-bin/corpgroup/corp/get_agent_info
    pub async fn get_agent_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corpgroup/corp/get_agent_info", req).await
    }

    /// 获取上下游信息 POST /cgi-bin/corpgroup/unionlist
    pub async fn get_union_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corpgroup/unionlist", req).await
    }

    /// 获取对接规则 ID 列表 GET /cgi-bin/corpgroup/rule_info/list
    pub async fn list_rules(&self) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/corpgroup/rule_info/list", &[]).await
    }

    /// 获取对接规则详情 POST /cgi-bin/corpgroup/rule_info/get
    pub async fn get_rule(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corpgroup/rule_info/get", req).await
    }

    /// 新增对接规则 POST /cgi-bin/corpgroup/rule_info/add
    pub async fn add_rule(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corpgroup/rule_info/add", req).await
    }

    /// 更新对接规则 POST /cgi-bin/corpgroup/rule_info/update
    pub async fn update_rule(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/corpgroup/rule_info/update", req).await
    }

    /// 删除对接规则 POST /cgi-bin/corpgroup/rule_info/delete
    pub async fn delete_rule(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/corpgroup/rule_info/delete", req).await
    }

    /// 批量导入上下游联系人 POST /cgi-bin/corpgroup/contact/batchadd
    pub async fn batch_add_contact(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corpgroup/contact/batchadd", req).await
    }

    /// 获取异步任务结果 GET /cgi-bin/corpgroup/contact/result
    pub async fn get_task_result(&self, jobid: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/corpgroup/contact/result", &[("jobid", jobid)]).await
    }
}
