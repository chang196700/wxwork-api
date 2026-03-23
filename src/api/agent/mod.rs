use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 应用管理 API
pub struct AgentApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> AgentApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取应用 GET /cgi-bin/agent/get
    pub async fn get(&self, agentid: i64) -> Result<AgentDetailResponse> {
        let id_s = agentid.to_string();
        self.client
            .get("/cgi-bin/agent/get", &[("agentid", id_s.as_str())])
            .await
    }

    /// 获取应用列表 GET /cgi-bin/agent/list
    pub async fn list(&self) -> Result<AgentListResponse> {
        self.client.get("/cgi-bin/agent/list", &[]).await
    }

    /// 设置应用 POST /cgi-bin/agent/set
    pub async fn set(&self, req: &SetAgentRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/agent/set", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 设置工作台自定义展示 POST /cgi-bin/agent/set_workbench_template
    pub async fn set_workbench_template(&self, req: &serde_json::Value) -> Result<()> {
        let resp: BaseResponse = self
            .client
            .post("/cgi-bin/agent/set_workbench_template", req)
            .await?;
        WxWorkClient::check_base(resp)
    }

    /// 获取自定义菜单 GET /cgi-bin/menu/get
    pub async fn get_menu(&self, agentid: i64) -> Result<MenuResponse> {
        let id_s = agentid.to_string();
        self.client
            .get("/cgi-bin/menu/get", &[("agentid", id_s.as_str())])
            .await
    }

    /// 创建菜单 POST /cgi-bin/menu/create
    pub async fn create_menu(&self, agentid: i64, button: &serde_json::Value) -> Result<()> {
        let body = serde_json::json!({ "agentid": agentid, "button": button });
        let resp: BaseResponse = self.client.post("/cgi-bin/menu/create", &body).await?;
        WxWorkClient::check_base(resp)
    }

    /// 删除菜单 GET /cgi-bin/menu/delete
    pub async fn delete_menu(&self, agentid: i64) -> Result<()> {
        let id_s = agentid.to_string();
        let resp: BaseResponse = self
            .client
            .get("/cgi-bin/menu/delete", &[("agentid", id_s.as_str())])
            .await?;
        WxWorkClient::check_base(resp)
    }
}

// ============ Request types ============

#[derive(Debug, Serialize)]
pub struct SetAgentRequest {
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_location_flag: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_mediaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isreportenter: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct AgentDetailResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub agentid: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub allow_userinfos: Option<serde_json::Value>,
    pub allow_partys: Option<serde_json::Value>,
    pub allow_tags: Option<serde_json::Value>,
    pub close: Option<u8>,
    pub redirect_domain: Option<String>,
    pub report_location_flag: Option<u8>,
    pub isreportenter: Option<u8>,
    pub home_url: Option<String>,
    pub square_logo_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgentListResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub agentlist: Vec<AgentItem>,
}

#[derive(Debug, Deserialize)]
pub struct AgentItem {
    pub agentid: i64,
    pub name: Option<String>,
    pub square_logo_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MenuResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub button: Option<serde_json::Value>,
}
