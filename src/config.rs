/// 企业微信 API 配置
#[derive(Debug, Clone)]
pub struct WxWorkConfig {
    /// 企业 ID
    pub corp_id: String,
    /// 应用 Secret（每个应用独立）
    pub corp_secret: String,
    /// 企业微信 API 基础 URL，默认 https://qyapi.weixin.qq.com
    pub base_url: String,
    /// 代理配置
    pub proxy: ProxyConfig,
    /// HTTP 请求超时（秒），默认 30
    pub timeout_secs: u64,
}

impl WxWorkConfig {
    pub fn new(corp_id: impl Into<String>, corp_secret: impl Into<String>) -> Self {
        Self {
            corp_id: corp_id.into(),
            corp_secret: corp_secret.into(),
            base_url: "https://qyapi.weixin.qq.com".to_string(),
            proxy: ProxyConfig::None,
            timeout_secs: 30,
        }
    }

    pub fn with_proxy(mut self, proxy: ProxyConfig) -> Self {
        self.proxy = proxy;
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// 返回唯一的 token 缓存 key（corpid + secret 的前 8 字符）
    pub fn token_key(&self) -> String {
        format!("{}:{}", self.corp_id, &self.corp_secret[..8.min(self.corp_secret.len())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = WxWorkConfig::new("corpid", "secret");
        assert_eq!(cfg.corp_id, "corpid");
        assert_eq!(cfg.corp_secret, "secret");
        assert_eq!(cfg.base_url, "https://qyapi.weixin.qq.com");
        assert_eq!(cfg.timeout_secs, 30);
        assert!(matches!(cfg.proxy, ProxyConfig::None));
    }

    #[test]
    fn test_builder_chain() {
        let cfg = WxWorkConfig::new("c", "s")
            .with_timeout(60)
            .with_base_url("https://example.com")
            .with_proxy(ProxyConfig::http("http://proxy:8080"));
        assert_eq!(cfg.timeout_secs, 60);
        assert_eq!(cfg.base_url, "https://example.com");
        assert!(matches!(cfg.proxy, ProxyConfig::Http(_)));
    }

    #[test]
    fn test_token_key_full_secret() {
        let cfg = WxWorkConfig::new("corp123", "abcdefghijklmn");
        assert_eq!(cfg.token_key(), "corp123:abcdefgh");
    }

    #[test]
    fn test_token_key_short_secret() {
        let cfg = WxWorkConfig::new("corp123", "abc");
        assert_eq!(cfg.token_key(), "corp123:abc");
    }

    #[test]
    fn test_proxy_constructors() {
        let p = ProxyConfig::http("http://h:80");
        assert!(matches!(p, ProxyConfig::Http(_)));

        let p = ProxyConfig::https("https://h:443");
        assert!(matches!(p, ProxyConfig::Https(_)));

        let p = ProxyConfig::socks5("socks5://h:1080");
        assert!(matches!(p, ProxyConfig::Socks5(_)));

        let p = ProxyConfig::with_auth("socks5://h:1080", "user", "pass");
        assert!(matches!(p, ProxyConfig::WithAuth { .. }));
    }
}

/// 代理配置
#[derive(Debug, Clone, Default)]
pub enum ProxyConfig {
    /// 不使用代理
    #[default]
    None,
    /// HTTP 代理，例如 http://127.0.0.1:8080
    Http(String),
    /// HTTPS 代理，例如 https://127.0.0.1:8080
    Https(String),
    /// SOCKS5 代理，例如 socks5://127.0.0.1:1080
    Socks5(String),
    /// 带认证的代理（支持 http / https / socks5）
    WithAuth {
        url: String,
        username: String,
        password: String,
    },
}

impl ProxyConfig {
    /// 创建 HTTP 代理
    pub fn http(url: impl Into<String>) -> Self {
        Self::Http(url.into())
    }

    /// 创建 HTTPS 代理
    pub fn https(url: impl Into<String>) -> Self {
        Self::Https(url.into())
    }

    /// 创建 SOCKS5 代理
    pub fn socks5(url: impl Into<String>) -> Self {
        Self::Socks5(url.into())
    }

    /// 创建带认证的代理
    pub fn with_auth(
        url: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self::WithAuth {
            url: url.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}
