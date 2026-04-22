use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 通讯录管理 - 成员管理 API
pub struct MemberApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> MemberApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 创建成员 POST /cgi-bin/user/create
    pub async fn create(&self, req: &CreateMemberRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/user/create", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 读取成员 GET /cgi-bin/user/get
    pub async fn get(&self, userid: &str) -> Result<MemberDetail> {
        self.client
            .get("/cgi-bin/user/get", &[("userid", userid)])
            .await
    }

    /// 更新成员 POST /cgi-bin/user/update
    pub async fn update(&self, req: &UpdateMemberRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/user/update", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 删除成员 GET /cgi-bin/user/delete
    pub async fn delete(&self, userid: &str) -> Result<()> {
        let resp: BaseResponse = self
            .client
            .get("/cgi-bin/user/delete", &[("userid", userid)])
            .await?;
        WxWorkClient::check_base(resp)
    }

    /// 批量删除成员 POST /cgi-bin/user/batchdelete
    pub async fn batch_delete(&self, useridlist: Vec<String>) -> Result<()> {
        let body = serde_json::json!({ "useridlist": useridlist });
        let resp: BaseResponse = self.client.post("/cgi-bin/user/batchdelete", &body).await?;
        WxWorkClient::check_base(resp)
    }

    /// 获取部门成员 GET /cgi-bin/user/simplelist
    pub async fn list_simple(
        &self,
        department_id: u64,
        fetch_child: Option<u8>,
    ) -> Result<SimpleUserListResponse> {
        let department_id_s = department_id.to_string();
        let fetch_child_s = fetch_child.unwrap_or(0).to_string();
        self.client
            .get(
                "/cgi-bin/user/simplelist",
                &[
                    ("department_id", department_id_s.as_str()),
                    ("fetch_child", fetch_child_s.as_str()),
                ],
            )
            .await
    }

    /// 获取部门成员详情 GET /cgi-bin/user/list
    pub async fn list(
        &self,
        department_id: u64,
        fetch_child: Option<u8>,
    ) -> Result<UserListResponse> {
        let department_id_s = department_id.to_string();
        let fetch_child_s = fetch_child.unwrap_or(0).to_string();
        self.client
            .get(
                "/cgi-bin/user/list",
                &[
                    ("department_id", department_id_s.as_str()),
                    ("fetch_child", fetch_child_s.as_str()),
                ],
            )
            .await
    }

    /// 获取成员 ID 列表 POST /cgi-bin/user/list_id
    pub async fn list_id(&self, req: &ListIdRequest) -> Result<ListIdResponse> {
        self.client.post("/cgi-bin/user/list_id", req).await
    }

    /// 获取加入企业二维码 GET /cgi-bin/corp/get_join_qrcode
    pub async fn get_join_qrcode(&self, size_type: Option<u8>) -> Result<JoinQrcodeResponse> {
        let mut query: Vec<(&str, &str)> = vec![];
        let size_str;
        if let Some(s) = size_type {
            size_str = s.to_string();
            query.push(("size_type", &size_str));
        }
        self.client.get("/cgi-bin/corp/get_join_qrcode", &query).await
    }

    /// userid 与 openid 互换 POST /cgi-bin/user/convert_to_openid
    pub async fn convert_to_openid(&self, userid: &str) -> Result<OpenidResponse> {
        let body = serde_json::json!({ "userid": userid });
        self.client.post("/cgi-bin/user/convert_to_openid", &body).await
    }

    /// 手机号获取 userid POST /cgi-bin/user/getuserid
    pub async fn get_userid_by_mobile(&self, mobile: &str) -> Result<GetUseridResponse> {
        let body = serde_json::json!({ "mobile": mobile });
        self.client.post("/cgi-bin/user/getuserid", &body).await
    }

    /// 邮箱获取 userid POST /cgi-bin/user/get_userid_by_email
    ///
    /// `email_type`: 1-企业邮箱（默认）；2-个人邮箱
    pub async fn get_userid_by_email(&self, email: &str, email_type: Option<u8>) -> Result<GetUseridResponse> {
        let mut body = serde_json::json!({ "email": email });
        if let Some(t) = email_type {
            body["email_type"] = serde_json::json!(t);
        }
        self.client.post("/cgi-bin/user/get_userid_by_email", &body).await
    }
}

// ============ Request types ============

#[derive(Debug, Serialize, Default)]
pub struct CreateMemberRequest {
    pub userid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub department: Vec<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub order: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_mail: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub is_leader_in_dept: Vec<u8>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub direct_leader: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_mediaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_department: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extattr: Option<ExtAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_invite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_profile: Option<ExternalProfile>,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateMemberRequest {
    pub userid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_mail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leader_in_dept: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_leader: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_mediaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_department: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extattr: Option<ExtAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_profile: Option<ExternalProfile>,
}

#[derive(Debug, Serialize, Default)]
pub struct ListIdRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

// ============ Response types ============

#[derive(Debug, Deserialize)]
pub struct MemberDetail {
    pub errcode: i32,
    pub errmsg: String,
    pub userid: Option<String>,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub mobile: Option<String>,
    #[serde(default)]
    pub department: Vec<u64>,
    #[serde(default)]
    pub order: Vec<u32>,
    pub position: Option<String>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub biz_mail: Option<String>,
    #[serde(default)]
    pub is_leader_in_dept: Vec<u8>,
    #[serde(default)]
    pub direct_leader: Vec<String>,
    pub avatar: Option<String>,
    pub thumb_avatar: Option<String>,
    pub telephone: Option<String>,
    pub address: Option<String>,
    pub open_userid: Option<String>,
    pub main_department: Option<u64>,
    pub extattr: Option<ExtAttr>,
    pub status: Option<u8>,
    pub qr_code: Option<String>,
    pub external_position: Option<String>,
    pub external_profile: Option<ExternalProfile>,
    pub enable: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleUserListResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub userlist: Vec<SimpleUser>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleUser {
    pub userid: String,
    pub name: Option<String>,
    pub department: Option<Vec<u64>>,
    pub open_userid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserListResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub userlist: Vec<MemberDetail>,
}

#[derive(Debug, Deserialize)]
pub struct ListIdResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub dept_user: Option<Vec<DeptUser>>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeptUser {
    pub userid: String,
    pub department: u64,
}

#[derive(Debug, Deserialize)]
pub struct JoinQrcodeResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub join_qrcode: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OpenidResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub openid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetUseridResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub userid: Option<String>,
}

// ============ Shared types ============

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ExtAttr {
    #[serde(default)]
    pub attrs: Vec<ExtAttrItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtAttrItem {
    #[serde(rename = "type")]
    pub attr_type: u8,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ExtAttrText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<ExtAttrWeb>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtAttrText {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtAttrWeb {
    pub url: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ExternalProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_corp_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_channels: Option<WechatChannels>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub external_attr: Vec<ExtAttrItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatChannels {
    pub nickname: String,
    pub status: Option<u8>,
}
