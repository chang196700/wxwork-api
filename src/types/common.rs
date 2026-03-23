use serde::{Deserialize, Serialize};

/// 企业微信通用 API 响应包装
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(flatten)]
    pub data: Option<T>,
}

/// 仅含 errcode/errmsg 的基础响应（用于无返回数据的接口）
#[derive(Debug, Deserialize)]
pub struct BaseResponse {
    pub errcode: i32,
    pub errmsg: String,
}

/// access_token 响应
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,
}

/// 分页游标（nextcursor）
#[derive(Debug, Deserialize)]
pub struct CursorResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub next_cursor: Option<String>,
}

/// 通用带列表的响应
#[derive(Debug, Deserialize)]
pub struct ListResponse<T> {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub list: Vec<T>,
}

/// 媒体文件 media_id 响应
#[derive(Debug, Deserialize)]
pub struct MediaIdResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub media_id: Option<String>,
    pub created_at: Option<String>,
}

/// 异步任务 jobid 响应
#[derive(Debug, Deserialize)]
pub struct JobIdResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub jobid: Option<String>,
}

/// 通用分页参数
#[derive(Debug, Serialize, Default)]
pub struct PageParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// 通用游标分页参数
#[derive(Debug, Serialize, Default)]
pub struct CursorPageParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_response_ok() {
        let json = r#"{"errcode":0,"errmsg":"ok"}"#;
        let r: BaseResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.errcode, 0);
        assert_eq!(r.errmsg, "ok");
    }

    #[test]
    fn test_base_response_error() {
        let json = r#"{"errcode":40001,"errmsg":"invalid credential"}"#;
        let r: BaseResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.errcode, 40001);
    }

    #[test]
    fn test_access_token_response_ok() {
        let json = r#"{"errcode":0,"errmsg":"ok","access_token":"TOKEN_VALUE","expires_in":7200}"#;
        let r: AccessTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.errcode, 0);
        assert_eq!(r.access_token.unwrap(), "TOKEN_VALUE");
        assert_eq!(r.expires_in.unwrap(), 7200);
    }

    #[test]
    fn test_access_token_response_error() {
        let json = r#"{"errcode":40013,"errmsg":"invalid corpid"}"#;
        let r: AccessTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.errcode, 40013);
        assert!(r.access_token.is_none());
        assert!(r.expires_in.is_none());
    }

    #[test]
    fn test_list_response_default_empty() {
        let json = r#"{"errcode":0,"errmsg":"ok"}"#;
        let r: ListResponse<String> = serde_json::from_str(json).unwrap();
        assert!(r.list.is_empty());
    }

    #[test]
    fn test_list_response_with_items() {
        let json = r#"{"errcode":0,"errmsg":"ok","list":["a","b","c"]}"#;
        let r: ListResponse<String> = serde_json::from_str(json).unwrap();
        assert_eq!(r.list.len(), 3);
    }

    #[test]
    fn test_job_id_response() {
        let json = r#"{"errcode":0,"errmsg":"ok","jobid":"JOB123"}"#;
        let r: JobIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.jobid.unwrap(), "JOB123");
    }

    #[test]
    fn test_page_param_skips_none() {
        let p = PageParam::default();
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_page_param_with_values() {
        let p = PageParam { offset: Some(10), limit: Some(50) };
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["offset"], 10);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_cursor_page_param_skips_none() {
        let p = CursorPageParam::default();
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.as_object().unwrap().is_empty());
    }
}
