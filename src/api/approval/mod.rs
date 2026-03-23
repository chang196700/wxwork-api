use serde::{Deserialize, Serialize};
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 审批 API
pub struct ApprovalApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> ApprovalApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取审批模板详情 GET /cgi-bin/oa/gettemplatedetail
    pub async fn get_template_detail(&self, template_id: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/oa/gettemplatedetail", &[("template_id", template_id)]).await
    }

    /// 创建审批模板 POST /cgi-bin/oa/approval/create_template
    pub async fn create_template(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/oa/approval/create_template", req).await
    }

    /// 更新审批模板 POST /cgi-bin/oa/approval/update_template
    pub async fn update_template(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/oa/approval/update_template", req).await
    }

    /// 提交审批申请 POST /cgi-bin/oa/applyevent
    pub async fn apply_event(&self, req: &serde_json::Value) -> Result<ApplyEventResponse> {
        self.client.post("/cgi-bin/oa/applyevent", req).await
    }

    /// 批量获取审批单号 POST /cgi-bin/oa/getapprovalinfo
    pub async fn get_approval_info(&self, req: &GetApprovalInfoRequest) -> Result<ApprovalInfoResponse> {
        self.client.post("/cgi-bin/oa/getapprovalinfo", req).await
    }

    /// 获取审批申请详情 GET /cgi-bin/oa/getapprovaldetail
    pub async fn get_approval_detail(&self, sp_no: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/oa/getapprovaldetail", &[("sp_no", sp_no)]).await
    }

    /// 获取审批数据（旧接口）POST /cgi-bin/corp/getapprovaldata
    pub async fn get_approval_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/corp/getapprovaldata", req).await
    }

    /// 获取企业假期管理配置 GET /cgi-bin/oa/vacation/getcorpconf
    pub async fn get_corp_vacation_conf(&self) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/oa/vacation/getcorpconf", &[]).await
    }

    /// 获取成员假期余额 POST /cgi-bin/oa/vacation/getuservacationquota
    pub async fn get_user_vacation_quota(&self, userid: &str) -> Result<serde_json::Value> {
        let body = serde_json::json!({ "userid": userid });
        self.client.post("/cgi-bin/oa/vacation/getuservacationquota", &body).await
    }

    /// 修改成员假期余额 POST /cgi-bin/oa/vacation/setoneuservacationquota
    pub async fn set_user_vacation_quota(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/oa/vacation/setoneuservacationquota", req).await
    }
}

#[derive(Debug, Serialize)]
pub struct GetApprovalInfoRequest {
    pub starttime: i64,
    pub endtime: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ApprovalFilter>>,
}

#[derive(Debug, Serialize)]
pub struct ApprovalFilter {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplyEventResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub sp_no: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApprovalInfoResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub sp_no_list: Option<Vec<String>>,
}
