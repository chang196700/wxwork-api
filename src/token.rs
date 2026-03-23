use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::Client;
use tokio::sync::RwLock;

use crate::config::WxWorkConfig;
use crate::error::{Result, WxWorkError};
use crate::types::common::AccessTokenResponse;

/// 单个 token 缓存条目
struct TokenEntry {
    token: String,
    /// token 过期时间点（预留 5 分钟缓冲）
    expires_at: Instant,
}

/// access_token 管理器
///
/// - 线程安全（Arc<RwLock>）
/// - 同一个 corpid+secret 共享缓存
/// - token 有效期内直接返回缓存，过期或临近过期时自动刷新
pub struct TokenManager {
    config: Arc<WxWorkConfig>,
    http: Client,
    cache: RwLock<HashMap<String, TokenEntry>>,
}

impl TokenManager {
    pub fn new(config: Arc<WxWorkConfig>, http: Client) -> Self {
        Self {
            config,
            http,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// 获取有效的 access_token（优先从缓存读取，过期则重新获取）
    pub async fn get_token(&self) -> Result<String> {
        let key = self.config.token_key();

        // 尝试读缓存（读锁）
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(&key) {
                if Instant::now() < entry.expires_at {
                    return Ok(entry.token.clone());
                }
            }
        }

        // 缓存过期或不存在，重新请求（写锁）
        let mut cache = self.cache.write().await;
        // 双重检查：可能在等待写锁期间已被其他协程刷新
        if let Some(entry) = cache.get(&key) {
            if Instant::now() < entry.expires_at {
                return Ok(entry.token.clone());
            }
        }

        let entry = self.fetch_token().await?;
        let token = entry.token.clone();
        cache.insert(key, entry);
        Ok(token)
    }

    /// 强制刷新 token
    pub async fn refresh_token(&self) -> Result<String> {
        let key = self.config.token_key();
        let entry = self.fetch_token().await?;
        let token = entry.token.clone();
        let mut cache = self.cache.write().await;
        cache.insert(key, entry);
        Ok(token)
    }

    /// 清除缓存（用于测试或手动重置）
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// 从企业微信服务端获取 access_token
    async fn fetch_token(&self) -> Result<TokenEntry> {
        let url = format!(
            "{}/cgi-bin/gettoken",
            self.config.base_url
        );

        let resp: AccessTokenResponse = self
            .http
            .get(&url)
            .query(&[
                ("corpid", self.config.corp_id.as_str()),
                ("corpsecret", self.config.corp_secret.as_str()),
            ])
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .send()
            .await
            .map_err(WxWorkError::Http)?
            .json()
            .await
            .map_err(WxWorkError::Http)?;

        if resp.errcode != 0 {
            return Err(WxWorkError::TokenError(format!(
                "errcode={}, errmsg={}",
                resp.errcode, resp.errmsg
            )));
        }

        let token = resp
            .access_token
            .ok_or_else(|| WxWorkError::TokenError("access_token missing in response".to_string()))?;
        let expires_in = resp.expires_in.unwrap_or(7200);

        // 提前 5 分钟过期，防止边界情况
        let effective_secs = if expires_in > 300 { expires_in - 300 } else { expires_in };
        let expires_at = Instant::now() + Duration::from_secs(effective_secs);

        tracing::debug!("access_token refreshed, expires_in={}s", expires_in);

        Ok(TokenEntry { token, expires_at })
    }
}
