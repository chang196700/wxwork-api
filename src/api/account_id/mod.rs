use serde::Deserialize;
use crate::client::WxWorkClient;
use crate::error::Result;

/// 账号 ID 转换 API
pub struct AccountIdApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> AccountIdApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// userid 转换 POST /cgi-bin/userid/translate
    pub async fn translate_userid(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/userid/translate", req).await
    }

    /// 邮箱/手机号获取 userid POST /cgi-bin/user/get_userid
    pub async fn get_userid_by_email_or_mobile(
        &self,
        req: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/user/get_userid", req).await
    }

    /// openid 转换 userid POST /cgi-bin/user/convert_to_userid
    pub async fn convert_to_userid(&self, openid: &str) -> Result<ConvertToUseridResponse> {
        let body = serde_json::json!({ "openid": openid });
        self.client.post("/cgi-bin/user/convert_to_userid", &body).await
    }
}

#[derive(Debug, Deserialize)]
pub struct ConvertToUseridResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub userid: Option<String>,
}
