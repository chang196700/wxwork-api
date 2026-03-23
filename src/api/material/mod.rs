use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::MediaIdResponse;

/// 素材管理 API
pub struct MaterialApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> MaterialApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 上传临时素材（multipart）POST /cgi-bin/media/upload?type=TYPE
    pub async fn upload_temp(&self, media_type: &str, form: reqwest::multipart::Form) -> Result<MediaIdResponse> {
        let path = format!("/cgi-bin/media/upload?type={}", media_type);
        self.client.upload(&path, form).await
    }

    /// 获取临时素材 GET /cgi-bin/media/get
    pub async fn get_temp_media_bytes(&self, media_id: &str) -> Result<bytes::Bytes> {
        let token = self.client.access_token().await?;
        let url = format!("{}/cgi-bin/media/get", self.client.config.base_url);
        let bytes = self
            .client
            .http
            .get(&url)
            .query(&[("access_token", token.as_str()), ("media_id", media_id)])
            .send()
            .await
            .map_err(crate::error::WxWorkError::Http)?
            .bytes()
            .await
            .map_err(crate::error::WxWorkError::Http)?;
        Ok(bytes)
    }

    /// 上传图片 POST /cgi-bin/media/uploadimg
    pub async fn upload_image(&self, form: reqwest::multipart::Form) -> Result<UploadImageResponse> {
        self.client.upload("/cgi-bin/media/uploadimg", form).await
    }
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct UploadImageResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub url: Option<String>,
}
