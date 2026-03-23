use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::JobIdResponse;

/// 通讯录管理 - 异步导出接口
pub struct ContactExportApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> ContactExportApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 导出成员 POST /cgi-bin/export/simple_user
    pub async fn export_simple_user(&self, req: &ExportRequest) -> Result<JobIdResponse> {
        self.client.post("/cgi-bin/export/simple_user", req).await
    }

    /// 导出成员详情 POST /cgi-bin/export/user
    pub async fn export_user(&self, req: &ExportRequest) -> Result<JobIdResponse> {
        self.client.post("/cgi-bin/export/user", req).await
    }

    /// 导出部门 POST /cgi-bin/export/department
    pub async fn export_department(&self, req: &ExportRequest) -> Result<JobIdResponse> {
        self.client.post("/cgi-bin/export/department", req).await
    }

    /// 导出标签成员 POST /cgi-bin/export/taguser
    pub async fn export_tag_user(&self, req: &ExportTagRequest) -> Result<JobIdResponse> {
        self.client.post("/cgi-bin/export/taguser", req).await
    }

    /// 获取导出结果 GET /cgi-bin/export/get_result
    pub async fn get_result(&self, jobid: &str) -> Result<ExportResultResponse> {
        self.client
            .get("/cgi-bin/export/get_result", &[("jobid", jobid)])
            .await
    }
}

/// 通讯录管理 - 异步导入接口
pub struct ContactImportApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> ContactImportApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 增量更新成员 POST /cgi-bin/batch/syncuser
    pub async fn sync_user(&self, req: &SyncUserRequest) -> Result<JobIdResponse> {
        self.client.post("/cgi-bin/batch/syncuser", req).await
    }

    /// 全量覆盖成员 POST /cgi-bin/batch/replaceuser
    pub async fn replace_user(&self, req: &ReplaceUserRequest) -> Result<JobIdResponse> {
        self.client.post("/cgi-bin/batch/replaceuser", req).await
    }

    /// 全量覆盖部门 POST /cgi-bin/batch/replaceparty
    pub async fn replace_party(&self, req: &ReplacePartyRequest) -> Result<JobIdResponse> {
        self.client.post("/cgi-bin/batch/replaceparty", req).await
    }

    /// 获取异步任务结果 GET /cgi-bin/batch/getresult
    pub async fn get_result(&self, jobid: &str) -> Result<BatchResultResponse> {
        self.client
            .get("/cgi-bin/batch/getresult", &[("jobid", jobid)])
            .await
    }
}

// ============ Request types ============

#[derive(Debug, Serialize, Default)]
pub struct ExportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_aeskey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ExportTagRequest {
    pub tagid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_aeskey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SyncUserRequest {
    pub media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_invite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<BatchCallback>,
}

#[derive(Debug, Serialize)]
pub struct ReplaceUserRequest {
    pub media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_invite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<BatchCallback>,
}

#[derive(Debug, Serialize)]
pub struct ReplacePartyRequest {
    pub media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<BatchCallback>,
}

#[derive(Debug, Serialize)]
pub struct BatchCallback {
    pub url: String,
    pub token: String,
    pub encodingaeskey: String,
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct ExportResultResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub status: Option<u8>,
    pub type_field: Option<u8>,
    pub total: Option<u32>,
    pub percentage: Option<u32>,
    pub remaintime: Option<u32>,
    #[serde(default)]
    pub file_list: Vec<ExportFile>,
}

#[derive(Debug, Deserialize)]
pub struct ExportFile {
    pub url: Option<String>,
    pub size: Option<u64>,
    pub md5: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchResultResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub status: Option<u8>,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub total: Option<u32>,
    pub percentage: Option<u32>,
    pub remaintime: Option<u32>,
    pub result: Option<Vec<BatchResult>>,
}

#[derive(Debug, Deserialize)]
pub struct BatchResult {
    pub userid: Option<String>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>,
}
