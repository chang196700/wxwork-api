use serde::Deserialize;

use crate::client::WxWorkClient;
use crate::error::Result;

/// 身份验证 API（OAuth2 网页授权）
pub struct IdentityApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> IdentityApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取访问用户身份 GET /cgi-bin/user/getuserinfo
    pub async fn get_user_info(&self, code: &str) -> Result<UserInfoResponse> {
        self.client
            .get("/cgi-bin/user/getuserinfo", &[("code", code)])
            .await
    }

    /// 获取访问用户敏感信息 POST /cgi-bin/auth/getuserdetail
    pub async fn get_user_detail(&self, user_ticket: &str) -> Result<UserDetailResponse> {
        let body = serde_json::json!({ "user_ticket": user_ticket });
        self.client.post("/cgi-bin/auth/getuserdetail", &body).await
    }

    /// 获取企业 jsapi_ticket GET /cgi-bin/ticket/get
    pub async fn get_jsapi_ticket(&self) -> Result<JsapiTicketResponse> {
        self.client
            .get("/cgi-bin/ticket/get", &[("type", "jsapi")])
            .await
    }

    /// 获取应用 jsapi_ticket GET /cgi-bin/ticket/get?type=agent_config
    pub async fn get_agent_jsapi_ticket(&self) -> Result<JsapiTicketResponse> {
        self.client
            .get("/cgi-bin/ticket/get", &[("type", "agent_config")])
            .await
    }
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub userid: Option<String>,
    pub user_ticket: Option<String>,
    pub expires_in: Option<u64>,
    pub open_userid: Option<String>,
    pub external_userid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserDetailResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub userid: Option<String>,
    pub gender: Option<String>,
    pub avatar: Option<String>,
    pub qr_code: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub biz_mail: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JsapiTicketResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub ticket: Option<String>,
    pub expires_in: Option<u64>,
}
