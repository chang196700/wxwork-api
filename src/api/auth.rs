use serde::Deserialize;

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::AccessTokenResponse;

/// 开发指南相关 API
pub struct AuthApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> AuthApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取 access_token（自动缓存，建议直接使用 client.access_token()）
    pub async fn get_access_token(&self) -> Result<AccessTokenResponse> {
        self.client
            .get_raw::<AccessTokenResponse>(
                "/cgi-bin/gettoken",
                &[
                    ("corpid", &self.client.config.corp_id),
                    ("corpsecret", &self.client.config.corp_secret),
                ],
            )
            .await
    }

    /// 获取企业微信接口调用 IP 段
    pub async fn get_api_domain_ip(&self) -> Result<IpListResponse> {
        self.client
            .get::<IpListResponse>("/cgi-bin/get_api_domain_ip", &[])
            .await
    }

    /// 获取企业微信回调服务 IP 段
    pub async fn get_callback_ip(&self) -> Result<IpListResponse> {
        self.client
            .get::<IpListResponse>("/cgi-bin/getcallbackip", &[])
            .await
    }
}

/// IP 段列表响应
#[derive(Debug, Deserialize)]
pub struct IpListResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub ip_list: Option<Vec<String>>,
}
