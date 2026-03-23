use thiserror::Error;

use crate::error_codes::errcode_description;

/// 企业微信 API 错误类型
#[derive(Debug, Error)]
pub enum WxWorkError {
    /// HTTP 请求错误
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// 企业微信 API 返回的业务错误码
    #[error("WxWork API error {errcode}: {errmsg}")]
    ApiError {
        errcode: i32,
        errmsg: String,
    },

    /// 代理配置错误
    #[error("Proxy configuration error: {0}")]
    ProxyError(String),

    /// URL 解析错误
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// Token 获取失败
    #[error("Failed to acquire access_token: {0}")]
    TokenError(String),

    /// 配置缺失
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// 加解密错误
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// 其他错误
    #[error("Unexpected error: {0}")]
    Other(String),
}

impl WxWorkError {
    pub fn api(errcode: i32, errmsg: impl Into<String>) -> Self {
        Self::ApiError {
            errcode,
            errmsg: errmsg.into(),
        }
    }

    /// 返回错误码对应的中文说明（来自全局错误码表），若未知则返回 None
    pub fn description(&self) -> Option<&'static str> {
        match self {
            WxWorkError::ApiError { errcode, .. } => errcode_description(*errcode),
            _ => None,
        }
    }

    /// 是否是 token 过期错误（errcode = 42001 / 40014）
    pub fn is_token_expired(&self) -> bool {
        matches!(
            self,
            WxWorkError::ApiError { errcode, .. } if *errcode == 42001 || *errcode == 40014
        )
    }
}

pub type Result<T> = std::result::Result<T, WxWorkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let e = WxWorkError::api(40001, "invalid credential");
        assert_eq!(e.to_string(), "WxWork API error 40001: invalid credential");
    }

    #[test]
    fn test_is_token_expired_42001() {
        let e = WxWorkError::api(42001, "access_token expired");
        assert!(e.is_token_expired());
    }

    #[test]
    fn test_is_token_expired_40014() {
        let e = WxWorkError::api(40014, "invalid access_token");
        assert!(e.is_token_expired());
    }

    #[test]
    fn test_is_not_token_expired() {
        let e = WxWorkError::api(40001, "other error");
        assert!(!e.is_token_expired());
    }

    #[test]
    fn test_token_error_not_expired() {
        let e = WxWorkError::TokenError("some error".to_string());
        assert!(!e.is_token_expired());
    }

    #[test]
    fn test_config_error_display() {
        let e = WxWorkError::ConfigError("missing field".to_string());
        assert_eq!(e.to_string(), "Configuration error: missing field");
    }

    #[test]
    fn test_proxy_error_display() {
        let e = WxWorkError::ProxyError("bad url".to_string());
        assert_eq!(e.to_string(), "Proxy configuration error: bad url");
    }

    #[test]
    fn test_error_description_known() {
        let e = WxWorkError::api(40001, "invalid");
        assert_eq!(e.description(), Some("不合法的secret参数"));
    }

    #[test]
    fn test_error_description_unknown() {
        let e = WxWorkError::api(999999, "unknown");
        assert_eq!(e.description(), None);
    }

    #[test]
    fn test_non_api_error_no_description() {
        let e = WxWorkError::TokenError("err".to_string());
        assert_eq!(e.description(), None);
    }
}
