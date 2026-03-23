use std::sync::Arc;
use std::time::Duration;

use reqwest::{Client, Proxy};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::{ProxyConfig, WxWorkConfig};
use crate::error::{Result, WxWorkError};
use crate::token::TokenManager;
use crate::types::common::BaseResponse;

/// 企业微信 API 客户端
///
/// 封装 reqwest::Client，支持 HTTP/HTTPS/SOCKS5 代理，
/// 内置 access_token 自动管理。
#[derive(Clone)]
pub struct WxWorkClient {
    pub(crate) config: Arc<WxWorkConfig>,
    pub(crate) http: Client,
    pub(crate) token_mgr: Arc<TokenManager>,
}

impl WxWorkClient {
    /// 创建客户端
    pub fn new(config: WxWorkConfig) -> Result<Self> {
        let http = build_http_client(&config)?;
        let config = Arc::new(config);
        let token_mgr = Arc::new(TokenManager::new(Arc::clone(&config), http.clone()));
        Ok(Self { config, http, token_mgr })
    }

    /// 获取当前有效的 access_token（自动缓存刷新）
    pub async fn access_token(&self) -> Result<String> {
        self.token_mgr.get_token().await
    }

    /// 发送 GET 请求（自动携带 access_token）
    pub async fn get<T>(&self, path: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let token = self.access_token().await?;
        let url = format!("{}{}", self.config.base_url, path);
        let mut q: Vec<(&str, String)> = vec![("access_token", token)];
        q.extend(query.iter().map(|(k, v)| (*k, v.to_string())));

        let resp = self
            .http
            .get(&url)
            .query(&q)
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 发送 POST JSON 请求（自动携带 access_token）
    pub async fn post<B, T>(&self, path: &str, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let token = self.access_token().await?;
        let url = format!("{}{}", self.config.base_url, path);

        let resp = self
            .http
            .post(&url)
            .query(&[("access_token", &token)])
            .json(body)
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 发送 GET 请求（不自动携带 token，用于获取 token 本身等场景）
    pub async fn get_raw<T>(&self, path: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, path);
        let resp = self
            .http
            .get(&url)
            .query(query)
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 发送 POST 请求（不自动携带 token）
    pub async fn post_raw<B, T>(&self, path: &str, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, path);
        let resp = self
            .http
            .post(&url)
            .json(body)
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 发送 multipart/form-data 上传（自动携带 access_token）
    pub async fn upload<T>(&self, path: &str, form: reqwest::multipart::Form) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let token = self.access_token().await?;
        let url = format!("{}{}", self.config.base_url, path);

        let resp = self
            .http
            .post(&url)
            .query(&[("access_token", &token)])
            .multipart(form)
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 检查企业微信 API 返回值，若 errcode != 0 则转为 WxWorkError
    pub fn check_response(errcode: i32, errmsg: &str) -> Result<()> {
        if errcode == 0 {
            Ok(())
        } else {
            Err(WxWorkError::api(errcode, errmsg))
        }
    }

    /// 检查 BaseResponse
    pub fn check_base(resp: BaseResponse) -> Result<()> {
        Self::check_response(resp.errcode, &resp.errmsg)
    }
}

/// 根据代理配置构建 reqwest::Client
fn build_http_client(config: &WxWorkConfig) -> Result<Client> {
    let mut builder = Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .user_agent("wxwork-api-rust/0.1");

    builder = match &config.proxy {
        ProxyConfig::None => builder.no_proxy(),
        ProxyConfig::Http(url) => {
            let proxy = Proxy::http(url.as_str())
                .map_err(|e| WxWorkError::ProxyError(e.to_string()))?;
            builder.proxy(proxy)
        }
        ProxyConfig::Https(url) => {
            let proxy = Proxy::https(url.as_str())
                .map_err(|e| WxWorkError::ProxyError(e.to_string()))?;
            builder.proxy(proxy)
        }
        ProxyConfig::Socks5(url) => {
            // reqwest feature "socks" 支持 socks5://
            let proxy = Proxy::all(url.as_str())
                .map_err(|e| WxWorkError::ProxyError(e.to_string()))?;
            builder.proxy(proxy)
        }
        ProxyConfig::WithAuth { url, username, password } => {
            let proxy = Proxy::all(url.as_str())
                .map_err(|e| WxWorkError::ProxyError(e.to_string()))?
                .basic_auth(username, password);
            builder.proxy(proxy)
        }
    };

    builder.build().map_err(WxWorkError::Http)
}
