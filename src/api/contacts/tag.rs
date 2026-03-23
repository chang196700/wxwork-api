use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 通讯录管理 - 标签管理 API
pub struct TagApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> TagApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 创建标签 POST /cgi-bin/tag/create
    pub async fn create(&self, req: &CreateTagRequest) -> Result<CreateTagResponse> {
        self.client.post("/cgi-bin/tag/create", req).await
    }

    /// 更新标签名字 POST /cgi-bin/tag/update
    pub async fn update(&self, req: &UpdateTagRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/tag/update", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 删除标签 GET /cgi-bin/tag/delete
    pub async fn delete(&self, tagid: u32) -> Result<()> {
        let tagid_s = tagid.to_string();
        let resp: BaseResponse = self
            .client
            .get("/cgi-bin/tag/delete", &[("tagid", tagid_s.as_str())])
            .await?;
        WxWorkClient::check_base(resp)
    }

    /// 获取标签成员 GET /cgi-bin/tag/get
    pub async fn get(&self, tagid: u32) -> Result<TagMembersResponse> {
        let tagid_s = tagid.to_string();
        self.client
            .get("/cgi-bin/tag/get", &[("tagid", tagid_s.as_str())])
            .await
    }

    /// 增加标签成员 POST /cgi-bin/tag/addtagusers
    pub async fn add_users(&self, req: &TagUsersRequest) -> Result<TagUsersResponse> {
        self.client.post("/cgi-bin/tag/addtagusers", req).await
    }

    /// 删除标签成员 POST /cgi-bin/tag/deltagusers
    pub async fn delete_users(&self, req: &TagUsersRequest) -> Result<TagUsersResponse> {
        self.client.post("/cgi-bin/tag/deltagusers", req).await
    }

    /// 获取标签列表 GET /cgi-bin/tag/list
    pub async fn list(&self) -> Result<TagListResponse> {
        self.client.get("/cgi-bin/tag/list", &[]).await
    }
}

// ============ Request types ============

#[derive(Debug, Serialize)]
pub struct CreateTagRequest {
    pub tagname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagid: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTagRequest {
    pub tagid: u32,
    pub tagname: String,
}

#[derive(Debug, Serialize)]
pub struct TagUsersRequest {
    pub tagid: u32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub userlist: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub partylist: Vec<u64>,
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct CreateTagResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub tagid: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TagMembersResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub userlist: Vec<TagUser>,
    #[serde(default)]
    pub partylist: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct TagUser {
    pub userid: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TagUsersResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub invalidlist: Option<String>,
    pub invalidparty: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize)]
pub struct TagListResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub taglist: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub tagid: u32,
    pub tagname: String,
}
