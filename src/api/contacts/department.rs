use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 通讯录管理 - 部门管理 API
pub struct DepartmentApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> DepartmentApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 创建部门 POST /cgi-bin/department/create
    pub async fn create(&self, req: &CreateDepartmentRequest) -> Result<CreateDepartmentResponse> {
        self.client.post("/cgi-bin/department/create", req).await
    }

    /// 更新部门 POST /cgi-bin/department/update
    pub async fn update(&self, req: &UpdateDepartmentRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/department/update", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 删除部门 GET /cgi-bin/department/delete
    pub async fn delete(&self, id: u64) -> Result<()> {
        let id_s = id.to_string();
        let resp: BaseResponse = self
            .client
            .get("/cgi-bin/department/delete", &[("id", id_s.as_str())])
            .await?;
        WxWorkClient::check_base(resp)
    }

    /// 获取部门列表 GET /cgi-bin/department/list
    pub async fn list(&self, id: Option<u64>) -> Result<DepartmentListResponse> {
        let id_s = id.map(|i| i.to_string());
        let mut query: Vec<(&str, &str)> = vec![];
        if let Some(ref s) = id_s {
            query.push(("id", s.as_str()));
        }
        self.client.get("/cgi-bin/department/list", &query).await
    }

    /// 获取单个部门详情 GET /cgi-bin/department/get
    pub async fn get(&self, id: u64) -> Result<DepartmentDetailResponse> {
        let id_s = id.to_string();
        self.client
            .get("/cgi-bin/department/get", &[("id", id_s.as_str())])
            .await
    }

    /// 获取子部门 ID 列表 GET /cgi-bin/department/simplelist
    pub async fn list_simple(&self, id: Option<u64>) -> Result<DepartmentIdListResponse> {
        let id_s = id.map(|i| i.to_string());
        let mut query: Vec<(&str, &str)> = vec![];
        if let Some(ref s) = id_s {
            query.push(("id", s.as_str()));
        }
        self.client.get("/cgi-bin/department/simplelist", &query).await
    }
}

// ============ Request types ============

#[derive(Debug, Serialize)]
pub struct CreateDepartmentRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    pub parentid: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct UpdateDepartmentRequest {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct CreateDepartmentResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub id: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct DepartmentListResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub department: Vec<Department>,
}

#[derive(Debug, Deserialize)]
pub struct DepartmentDetailResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub department: Option<Department>,
}

#[derive(Debug, Deserialize)]
pub struct DepartmentIdListResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub department_id: Vec<DepartmentId>,
}

#[derive(Debug, Deserialize)]
pub struct Department {
    pub id: u64,
    pub name: Option<String>,
    pub name_en: Option<String>,
    pub parentid: Option<u64>,
    pub order: Option<u32>,
    pub department_leader: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct DepartmentId {
    pub id: u64,
    pub parentid: u64,
    pub order: u32,
}
