use serde::{Deserialize, Serialize};

use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 邮件 API
pub struct MailApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> MailApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    // ====== 发送邮件 ======

    /// 发送邮件（普通/日程/会议均使用此接口）POST /cgi-bin/exmail/app/compose_send
    ///
    /// - 普通邮件：不传 `schedule` 和 `meeting` 字段
    /// - 日程邮件：填写 `schedule` 字段
    /// - 会议邮件：同时填写 `schedule` 和 `meeting` 字段
    pub async fn compose_send(&self, req: &ComposeMailRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/app/compose_send", req).await?;
        WxWorkClient::check_base(resp)
    }

    // ====== 邮箱账号管理 ======

    /// 获取成员邮箱账号 GET /cgi-bin/mail/get_user_email
    pub async fn get_user_email(&self, userid: &str) -> Result<GetUserEmailResponse> {
        self.client.get("/cgi-bin/mail/get_user_email", &[("userid", userid)]).await
    }

    /// 禁用/启用邮箱账号 POST /cgi-bin/exmail/account/act_email
    pub async fn act_email(&self, req: &ActEmailRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/account/act_email", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 获取邮件未读数 POST /cgi-bin/exmail/mail/get_newcount
    pub async fn get_new_count(&self, userid: &str) -> Result<GetNewCountResponse> {
        let body = serde_json::json!({ "userid": userid });
        self.client.post("/cgi-bin/exmail/mail/get_newcount", &body).await
    }

    // ====== 应用邮箱账号管理 ======

    /// 查询应用邮箱账号及别名 POST /cgi-bin/exmail/app/get_email_alias
    pub async fn get_app_email_alias(&self) -> Result<AppEmailAliasResponse> {
        let body = serde_json::json!({});
        self.client.post("/cgi-bin/exmail/app/get_email_alias", &body).await
    }

    /// 更新应用邮箱账号 POST /cgi-bin/exmail/app/update_email_alias
    pub async fn update_app_email_alias(&self, new_email: &str) -> Result<()> {
        let body = serde_json::json!({ "new_email": new_email });
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/app/update_email_alias", &body).await?;
        WxWorkClient::check_base(resp)
    }

    // ====== 公共邮箱管理 ======

    /// 创建公共邮箱 POST /cgi-bin/exmail/publicmail/create
    pub async fn create_public_mail(&self, req: &CreatePublicMailRequest) -> Result<CreatePublicMailResponse> {
        self.client.post("/cgi-bin/exmail/publicmail/create", req).await
    }

    /// 获取公共邮箱详情 POST /cgi-bin/exmail/publicmail/get
    pub async fn get_public_mail(&self, id_list: &[u32]) -> Result<GetPublicMailResponse> {
        let body = serde_json::json!({ "id_list": id_list });
        self.client.post("/cgi-bin/exmail/publicmail/get", &body).await
    }

    /// 更新公共邮箱 POST /cgi-bin/exmail/publicmail/update
    pub async fn update_public_mail(&self, req: &UpdatePublicMailRequest) -> Result<UpdatePublicMailResponse> {
        self.client.post("/cgi-bin/exmail/publicmail/update", req).await
    }

    /// 删除公共邮箱 POST /cgi-bin/exmail/publicmail/delete
    pub async fn delete_public_mail(&self, id: u32) -> Result<()> {
        let body = serde_json::json!({ "id": id });
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/publicmail/delete", &body).await?;
        WxWorkClient::check_base(resp)
    }

    /// 模糊搜索公共邮箱 GET /cgi-bin/exmail/publicmail/search
    ///
    /// `fuzzy`: 1-开启模糊搜索，0-获取全部
    pub async fn search_public_mail(&self, fuzzy: u32, email: Option<&str>) -> Result<SearchPublicMailResponse> {
        let fuzzy_s = fuzzy.to_string();
        let mut query: Vec<(&str, &str)> = vec![("fuzzy", &fuzzy_s)];
        if let Some(e) = email {
            query.push(("email", e));
        }
        self.client.get("/cgi-bin/exmail/publicmail/search", &query).await
    }

    /// 获取公共邮箱客户端专用密码列表 POST /cgi-bin/exmail/publicmail/get_auth_code_list
    pub async fn get_public_mail_auth_codes(&self, id: u32) -> Result<GetAuthCodeListResponse> {
        let body = serde_json::json!({ "id": id });
        self.client.post("/cgi-bin/exmail/publicmail/get_auth_code_list", &body).await
    }

    /// 删除公共邮箱客户端专用密码 POST /cgi-bin/exmail/publicmail/delete_auth_code
    pub async fn delete_public_mail_auth_code(&self, id: u32, auth_code_id: u32) -> Result<()> {
        let body = serde_json::json!({ "id": id, "auth_code_id": auth_code_id });
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/publicmail/delete_auth_code", &body).await?;
        WxWorkClient::check_base(resp)
    }

    // ====== 邮件群组管理 ======

    /// 创建邮件群组 POST /cgi-bin/exmail/group/create
    pub async fn create_mail_group(&self, req: &CreateMailGroupRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/group/create", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 获取邮件群组详情 GET /cgi-bin/exmail/group/get
    pub async fn get_mail_group(&self, groupid: &str) -> Result<MailGroupDetail> {
        self.client.get("/cgi-bin/exmail/group/get", &[("groupid", groupid)]).await
    }

    /// 更新邮件群组 POST /cgi-bin/exmail/group/update
    pub async fn update_mail_group(&self, req: &UpdateMailGroupRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/group/update", req).await?;
        WxWorkClient::check_base(resp)
    }

    /// 删除邮件群组 POST /cgi-bin/exmail/group/delete
    pub async fn delete_mail_group(&self, groupid: &str) -> Result<()> {
        let body = serde_json::json!({ "groupid": groupid });
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/group/delete", &body).await?;
        WxWorkClient::check_base(resp)
    }

    /// 模糊搜索邮件群组 GET /cgi-bin/exmail/group/search
    ///
    /// `fuzzy`: 1-开启模糊搜索，0-获取全部
    pub async fn search_mail_group(&self, fuzzy: u32, groupid: Option<&str>) -> Result<SearchMailGroupResponse> {
        let fuzzy_s = fuzzy.to_string();
        let mut query: Vec<(&str, &str)> = vec![("fuzzy", &fuzzy_s)];
        if let Some(g) = groupid {
            query.push(("groupid", g));
        }
        self.client.get("/cgi-bin/exmail/group/search", &query).await
    }

    // ====== 获取接收的邮件 ======

    /// 获取收件箱邮件列表 POST /cgi-bin/exmail/app/get_mail_list
    pub async fn get_mail_list(&self, req: &GetMailListRequest) -> Result<GetMailListResponse> {
        self.client.post("/cgi-bin/exmail/app/get_mail_list", req).await
    }

    /// 获取邮件内容（eml格式）POST /cgi-bin/exmail/app/read_mail
    pub async fn read_mail(&self, mail_id: &str) -> Result<ReadMailResponse> {
        let body = serde_json::json!({ "mail_id": mail_id });
        self.client.post("/cgi-bin/exmail/app/read_mail", &body).await
    }

    // ====== 其他邮件客户端登录设置 ======

    /// 获取用户功能属性 POST /cgi-bin/exmail/useroption/get
    pub async fn get_user_option(&self, req: &GetUserOptionRequest) -> Result<GetUserOptionResponse> {
        self.client.post("/cgi-bin/exmail/useroption/get", req).await
    }

    /// 更改用户功能属性 POST /cgi-bin/exmail/useroption/update
    pub async fn update_user_option(&self, req: &UpdateUserOptionRequest) -> Result<()> {
        let resp: BaseResponse = self.client.post("/cgi-bin/exmail/useroption/update", req).await?;
        WxWorkClient::check_base(resp)
    }

    // ====== 高级功能账号管理 ======

    /// 分配高级功能账号 POST /cgi-bin/exmail/vip/batch_add
    ///
    /// 单次最多 100 个 userid
    pub async fn vip_batch_add(&self, userid_list: &[&str]) -> Result<VipBatchResponse> {
        let body = serde_json::json!({ "userid_list": userid_list });
        self.client.post("/cgi-bin/exmail/vip/batch_add", &body).await
    }

    /// 取消高级功能账号 POST /cgi-bin/exmail/vip/batch_del
    ///
    /// 单次最多 100 个 userid
    pub async fn vip_batch_del(&self, userid_list: &[&str]) -> Result<VipBatchResponse> {
        let body = serde_json::json!({ "userid_list": userid_list });
        self.client.post("/cgi-bin/exmail/vip/batch_del", &body).await
    }

    /// 获取高级功能账号列表 POST /cgi-bin/exmail/vip/list
    pub async fn vip_list(&self, cursor: Option<&str>, limit: Option<u32>) -> Result<VipListResponse> {
        let mut body = serde_json::Map::new();
        if let Some(c) = cursor {
            body.insert("cursor".to_string(), serde_json::Value::String(c.to_string()));
        }
        if let Some(l) = limit {
            body.insert("limit".to_string(), serde_json::Value::Number(l.into()));
        }
        self.client.post("/cgi-bin/exmail/vip/list", &serde_json::Value::Object(body)).await
    }

    // ====== 查询邮件发送任务 ======

    /// 查询邮件发送任务 POST /cgi-bin/mail/get_group_send_task
    pub async fn get_group_send_task(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/mail/get_group_send_task", req).await
    }
}

// ============================================================
// 共享辅助类型
// ============================================================

/// 字符串列表包装器（用于 `{list:[...]}` 格式的字段）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StringListWrapper {
    pub list: Vec<String>,
}

/// u32 列表包装器（用于 `{list:[...]}` 格式的部门/标签ID字段）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct U32ListWrapper {
    pub list: Vec<u32>,
}

// ============================================================
// 发送邮件请求类型
// ============================================================

/// 收件人/抄送/密送字段（emails 与 userids 至少填一个）
#[derive(Debug, Clone, Serialize, Default)]
pub struct MailRecipients {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub emails: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub userids: Vec<String>,
}

/// 邮件附件
#[derive(Debug, Clone, Serialize)]
pub struct MailAttachment {
    pub file_name: String,
    /// 文件内容（base64 编码），所有附件加正文不超过 50M，个数不超过 200
    pub content: String,
}

/// 日程/会议邮件中的重复提醒配置
#[derive(Debug, Clone, Serialize, Default)]
pub struct ScheduleReminders {
    /// 是否有提醒 0-不提醒 1-提醒
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_remind: Option<i32>,
    /// 开始前多少分钟提醒（负数表示开始后）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_before_event_mins: Option<i32>,
    /// 时区（UTC 偏移量，-12~+12，默认 8 即北京时间）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<i32>,
    /// 是否重复 0-否 1-是
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_repeat: Option<i32>,
    /// 是否自定义重复 0-否 1-是（is_repeat=1 时有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_custom_repeat: Option<i32>,
    /// 重复类型：0-每日 1-每周 2-每月 5-每年
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<i32>,
    /// 重复间隔（自定义重复时有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<i32>,
    /// 每周周几重复（1-7，自定义每周重复时有效）
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub repeat_day_of_week: Vec<i32>,
    /// 每月哪几天重复（1-31，自定义每月/每年时有效）
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub repeat_day_of_month: Vec<i32>,
    /// 每年哪些周（自定义每月时使用）
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub repeat_week_of_month: Vec<i32>,
    /// 每年哪几个月（1-12，自定义每年时有效）
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub repeat_month_of_year: Vec<i32>,
    /// 重复结束时间戳（0 或不填表示一直重复）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_until: Option<i64>,
}

/// 日程/会议邮件中的日程信息
#[derive(Debug, Clone, Serialize)]
pub struct MailSchedule {
    /// 日程/会议 ID（修改或取消时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// 方法：`request`（创建/修改，默认）或 `cancel`（取消，需传 schedule_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// 地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// 开始时间（Unix 时间戳）
    pub start_time: i64,
    /// 结束时间（Unix 时间戳）
    pub end_time: i64,
    /// 重复与提醒配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<ScheduleReminders>,
    /// 日程管理员（最多 3 人，仅日程邮件使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_admins: Option<UserIdList>,
}

/// userid 列表（用于 hosts/meeting_admins/schedule_admins）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdList {
    pub userids: Vec<String>,
}

/// 会议设置选项
#[derive(Debug, Clone, Serialize, Default)]
pub struct MeetingOption {
    /// 入会密码（4-6 位纯数字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 自动录制：0-不开启 1-本地录制 2-云录制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_record: Option<i32>,
    /// 是否开启等候室
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_waiting_room: Option<bool>,
    /// 是否允许成员在主持人前加入
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_enter_before_host: Option<bool>,
    /// 限制成员入会：0-所有人 2-仅企业内部
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_restraint: Option<i32>,
    /// 是否开启屏幕水印
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_screen_watermark: Option<bool>,
    /// 入会静音：0-关闭 1-开启 2-超6人自动开启
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_enter_mute: Option<i32>,
    /// 开始时提醒：1-不提醒 2-仅主持人 3-所有成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_scope: Option<i32>,
    /// 水印类型：0-单排 1-多排
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_mark_type: Option<i32>,
}

/// 会议邮件专属配置（需同时填写 `schedule` 字段）
#[derive(Debug, Clone, Serialize)]
pub struct MeetingConfig {
    /// 会议主持人（最多 10 人）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<UserIdList>,
    /// 会议管理员（仅 1 人，必须在参与人中）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_admins: Option<UserIdList>,
    /// 会议设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<MeetingOption>,
}

/// 发送邮件请求（普通/日程/会议共用此结构）
///
/// 有 `schedule` → 日程邮件；同时有 `schedule` 和 `meeting` → 会议邮件
#[derive(Debug, Clone, Serialize)]
pub struct ComposeMailRequest {
    /// 收件人（emails 与 userids 至少填一个）
    pub to: MailRecipients,
    /// 抄送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<MailRecipients>,
    /// 密送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<MailRecipients>,
    /// 邮件标题
    pub subject: String,
    /// 邮件正文
    pub content: String,
    /// 附件列表（所有附件加正文不超过 50M，个数不超过 200）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_list: Option<Vec<MailAttachment>>,
    /// 内容类型：`html`（默认）或 `text`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 日程/会议相关数据（日程邮件和会议邮件必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<MailSchedule>,
    /// 会议配置（会议邮件必填，须同时传 `schedule`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingConfig>,
    /// 是否开启 ID 转译（第三方应用用）：0-否 1-是
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<u32>,
}

// ============================================================
// 邮箱账号管理请求/响应
// ============================================================

/// 禁用/启用邮箱账号请求
#[derive(Debug, Clone, Serialize)]
pub struct ActEmailRequest {
    /// 成员 userid（与 publicemail_id 至少填一项，同时传则只操作 userid）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    /// 业务邮箱 ID（与 userid 至少填一项）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicemail_id: Option<u32>,
    /// 操作类型：1-启用 2-禁用
    #[serde(rename = "type")]
    pub act_type: u32,
}

#[derive(Debug, Deserialize)]
pub struct GetUserEmailResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub email: Option<String>,
    pub biz_mail: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetNewCountResponse {
    pub errcode: i32,
    pub errmsg: String,
    /// 未读邮件数
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AppEmailAliasResponse {
    pub errcode: i32,
    pub errmsg: String,
    /// 应用邮箱主地址（发件人地址）
    pub email: Option<String>,
    /// 别名邮箱列表（可作为收件人）
    pub alias_list: Option<Vec<String>>,
}

// ============================================================
// 公共邮箱管理请求/响应
// ============================================================

/// 客户端专用密码备注
#[derive(Debug, Clone, Serialize, Default)]
pub struct PublicMailAuthCodeInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 创建公共邮箱请求
#[derive(Debug, Clone, Serialize)]
pub struct CreatePublicMailRequest {
    pub email: String,
    pub name: String,
    /// 有权限的成员（userid_list/department_list/tag_list 不能同时为空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid_list: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<U32ListWrapper>,
    /// 是否创建客户端专用密码：0-否 1-是
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_auth_code: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code_info: Option<PublicMailAuthCodeInfo>,
}

/// 更新公共邮箱请求（传 `None` 表示不变；传空列表表示清空）
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePublicMailRequest {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid_list: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<U32ListWrapper>,
    /// 邮箱别名（最多 5 个，覆盖式更新，传空结构体则清空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_list: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_auth_code: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code_info: Option<PublicMailAuthCodeInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePublicMailResponse {
    pub errcode: i32,
    pub errmsg: String,
    /// 新建公共邮箱 ID
    pub id: Option<u32>,
    /// 客户端专用密码 ID（仅创建时返回）
    pub auth_code_id: Option<u32>,
    /// 客户端专用密码（仅创建时返回且只返回一次）
    pub auth_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePublicMailResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub auth_code_id: Option<u32>,
    pub auth_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PublicMailDetail {
    pub id: Option<u32>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub userid_list: Option<StringListWrapper>,
    pub department_list: Option<U32ListWrapper>,
    pub tag_list: Option<U32ListWrapper>,
    pub alias_list: Option<StringListWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct GetPublicMailResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub list: Option<Vec<PublicMailDetail>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchPublicMailItem {
    pub id: Option<u32>,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchPublicMailResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub list: Option<Vec<SearchPublicMailItem>>,
}

#[derive(Debug, Deserialize)]
pub struct AuthCodeItem {
    pub auth_code_id: Option<u32>,
    pub create_time: Option<u32>,
    /// 最后使用时间（未使用过则为 0）
    pub last_use_time: Option<u32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetAuthCodeListResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub auth_code_list: Option<Vec<AuthCodeItem>>,
}

// ============================================================
// 邮件群组管理请求/响应
// ============================================================

/// 创建邮件群组请求
#[derive(Debug, Clone, Serialize)]
pub struct CreateMailGroupRequest {
    /// 邮件群组 ID（邮箱格式）
    pub groupid: String,
    pub groupname: String,
    /// 群组成员邮箱（与 tag_list/department_list/group_list 至少填一项）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_list: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<StringListWrapper>,
    /// 使用权限：0-企业成员 1-任何人 2-组内成员 3-自定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_type: Option<u32>,
    /// 允许群发的成员邮箱（allow_type=3 时必填至少一项）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_emaillist: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_departmentlist: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_taglist: Option<U32ListWrapper>,
}

/// 更新邮件群组请求（传 `None` 表示不变；传空列表表示清空）
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMailGroupRequest {
    pub groupid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_list: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_emaillist: Option<StringListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_departmentlist: Option<U32ListWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_taglist: Option<U32ListWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct MailGroupDetail {
    pub errcode: i32,
    pub errmsg: String,
    pub groupid: Option<String>,
    pub groupname: Option<String>,
    pub email_list: Option<StringListWrapper>,
    pub tag_list: Option<U32ListWrapper>,
    pub department_list: Option<U32ListWrapper>,
    pub group_list: Option<StringListWrapper>,
    pub allow_type: Option<u32>,
    pub allow_emaillist: Option<StringListWrapper>,
    pub allow_departmentlist: Option<U32ListWrapper>,
    pub allow_taglist: Option<U32ListWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct MailGroupSummary {
    pub groupid: Option<String>,
    pub groupname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchMailGroupResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub count: Option<u32>,
    pub groups: Option<Vec<MailGroupSummary>>,
}

// ============================================================
// 收件箱邮件请求/响应
// ============================================================

/// 获取收件箱邮件列表请求
#[derive(Debug, Clone, Serialize)]
pub struct GetMailListRequest {
    /// 开始时间（Unix 时间戳）
    pub begin_time: u32,
    /// 结束时间（Unix 时间戳）
    pub end_time: u32,
    /// 分页游标（首次不填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// 每页条数（默认 100，最大 1000）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MailItem {
    pub mail_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetMailListResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub next_cursor: Option<String>,
    /// 是否还有更多：0-没有 1-有
    pub has_more: Option<u32>,
    pub mail_list: Option<Vec<MailItem>>,
}

#[derive(Debug, Deserialize)]
pub struct ReadMailResponse {
    pub errcode: i32,
    pub errmsg: String,
    /// 邮件 eml 内容（base64 编码或原始 eml 文本）
    pub mail_data: Option<String>,
}

// ============================================================
// 用户功能属性请求/响应
// ============================================================

/// 功能属性类型：1-强制启用安全登录 2-IMAP/SMTP 3-POP/SMTP 4-是否启用安全登录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOptionItem {
    #[serde(rename = "type")]
    pub option_type: u32,
    /// 1-启用 0-关闭
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOptionList {
    pub list: Vec<UserOptionItem>,
}

/// 获取用户功能属性请求
#[derive(Debug, Clone, Serialize)]
pub struct GetUserOptionRequest {
    pub userid: String,
    /// 要查询的属性类型列表
    #[serde(rename = "type")]
    pub option_types: Vec<u32>,
}

/// 更改用户功能属性请求
#[derive(Debug, Clone, Serialize)]
pub struct UpdateUserOptionRequest {
    pub userid: String,
    pub option: UserOptionList,
}

#[derive(Debug, Deserialize)]
pub struct GetUserOptionResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub option: Option<UserOptionList>,
}

// ============================================================
// 高级功能账号管理响应
// ============================================================

#[derive(Debug, Deserialize)]
pub struct VipBatchResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub succ_userid_list: Option<Vec<String>>,
    pub fail_userid_list: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct VipListResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub has_more: Option<bool>,
    pub next_cursor: Option<String>,
    pub userid_list: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── StringListWrapper / U32ListWrapper 序列化 ──────────────────────────

    #[test]
    fn test_string_list_wrapper_serialization() {
        let w = StringListWrapper { list: vec!["a".to_string(), "b".to_string()] };
        let json = serde_json::to_value(&w).unwrap();
        assert_eq!(json, serde_json::json!({"list": ["a", "b"]}));
    }

    #[test]
    fn test_string_list_wrapper_empty_serialization() {
        let w = StringListWrapper { list: vec![] };
        let json = serde_json::to_value(&w).unwrap();
        assert_eq!(json, serde_json::json!({"list": []}));
    }

    #[test]
    fn test_u32_list_wrapper_serialization() {
        let w = U32ListWrapper { list: vec![1, 2, 3] };
        let json = serde_json::to_value(&w).unwrap();
        assert_eq!(json, serde_json::json!({"list": [1, 2, 3]}));
    }

    #[test]
    fn test_string_list_wrapper_deserialization() {
        let json = r#"{"list":["x","y"]}"#;
        let w: StringListWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.list, vec!["x", "y"]);
    }

    // ── ComposeMailRequest 序列化 ─────────────────────────────────────────

    #[test]
    fn test_compose_mail_simple_no_optional_fields() {
        let req = ComposeMailRequest {
            to: MailRecipients {
                emails: vec!["user@example.com".to_string()],
                userids: vec![],
            },
            cc: None,
            bcc: None,
            subject: "Test subject".to_string(),
            content: "Hello".to_string(),
            attachment_list: None,
            content_type: None,
            schedule: None,
            meeting: None,
            enable_id_trans: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["subject"], "Test subject");
        assert_eq!(json["to"]["emails"][0], "user@example.com");
        // 空 vec 被 skip，不应出现在 to 中
        assert!(json["to"].get("userids").is_none());
        // 可选字段应被省略
        assert!(json.get("schedule").is_none());
        assert!(json.get("meeting").is_none());
        assert!(json.get("cc").is_none());
        assert!(json.get("enable_id_trans").is_none());
    }

    #[test]
    fn test_compose_mail_with_schedule() {
        let req = ComposeMailRequest {
            to: MailRecipients { emails: vec!["a@b.com".to_string()], userids: vec![] },
            cc: None,
            bcc: None,
            subject: "Schedule".to_string(),
            content: "Desc".to_string(),
            attachment_list: None,
            content_type: None,
            schedule: Some(MailSchedule {
                schedule_id: None,
                method: Some("request".to_string()),
                location: Some("Room A".to_string()),
                start_time: 1669278600,
                end_time: 1669282200,
                reminders: None,
                schedule_admins: None,
            }),
            meeting: None,
            enable_id_trans: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["schedule"]["start_time"], 1669278600i64);
        assert_eq!(json["schedule"]["location"], "Room A");
        assert_eq!(json["schedule"]["method"], "request");
        assert!(json["schedule"].get("schedule_id").is_none());
        assert!(json.get("meeting").is_none());
    }

    #[test]
    fn test_compose_mail_with_meeting() {
        let req = ComposeMailRequest {
            to: MailRecipients { emails: vec!["a@b.com".to_string()], userids: vec![] },
            cc: None,
            bcc: None,
            subject: "Meeting".to_string(),
            content: "Desc".to_string(),
            attachment_list: None,
            content_type: None,
            schedule: Some(MailSchedule {
                schedule_id: None,
                method: Some("request".to_string()),
                location: None,
                start_time: 1669278600,
                end_time: 1669282200,
                reminders: None,
                schedule_admins: None,
            }),
            meeting: Some(MeetingConfig {
                hosts: Some(UserIdList { userids: vec!["host1".to_string()] }),
                meeting_admins: Some(UserIdList { userids: vec!["admin1".to_string()] }),
                option: None,
            }),
            enable_id_trans: Some(1),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("schedule").is_some());
        assert_eq!(json["meeting"]["hosts"]["userids"][0], "host1");
        assert_eq!(json["meeting"]["meeting_admins"]["userids"][0], "admin1");
        assert!(json["meeting"].get("option").is_none());
        assert_eq!(json["enable_id_trans"], 1);
    }

    // ── UpdatePublicMailRequest：omit vs clear 语义 ──────────────────────

    #[test]
    fn test_update_public_mail_omit_fields() {
        // 不传 userid_list → JSON 中无该字段（保持不变）
        let req = UpdatePublicMailRequest {
            id: 1,
            name: Some("name".to_string()),
            userid_list: None,
            department_list: None,
            tag_list: None,
            alias_list: None,
            create_auth_code: None,
            auth_code_info: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("userid_list").is_none());
        assert!(json.get("department_list").is_none());
    }

    #[test]
    fn test_update_public_mail_clear_fields() {
        // 传空 list → JSON 中有 `{"list": []}` （清空）
        let req = UpdatePublicMailRequest {
            id: 1,
            name: None,
            userid_list: Some(StringListWrapper { list: vec![] }),
            department_list: Some(U32ListWrapper { list: vec![] }),
            tag_list: None,
            alias_list: None,
            create_auth_code: None,
            auth_code_info: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["userid_list"]["list"], serde_json::json!([]));
        assert_eq!(json["department_list"]["list"], serde_json::json!([]));
    }

    // ── CreateMailGroupRequest 序列化 ────────────────────────────────────

    #[test]
    fn test_create_mail_group_serialization() {
        let req = CreateMailGroupRequest {
            groupid: "group@example.com".to_string(),
            groupname: "Test Group".to_string(),
            email_list: Some(StringListWrapper { list: vec!["a@example.com".to_string()] }),
            tag_list: None,
            department_list: Some(U32ListWrapper { list: vec![1, 2] }),
            group_list: None,
            allow_type: Some(0),
            allow_emaillist: None,
            allow_departmentlist: None,
            allow_taglist: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["groupid"], "group@example.com");
        assert_eq!(json["email_list"]["list"][0], "a@example.com");
        assert_eq!(json["department_list"]["list"], serde_json::json!([1, 2]));
        assert!(json.get("tag_list").is_none());
    }

    // ── 响应反序列化 ─────────────────────────────────────────────────────

    #[test]
    fn test_get_public_mail_response_deserialization() {
        let json = r#"{
            "errcode": 0,
            "errmsg": "ok",
            "list": [{
                "id": 1,
                "email": "pub@example.com",
                "name": "PublicBox",
                "userid_list": {"list": ["user1", "user2"]},
                "department_list": {"list": [1, 2]},
                "tag_list": {"list": []},
                "alias_list": {"list": ["alias@example.com"]}
            }]
        }"#;
        let resp: GetPublicMailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.errcode, 0);
        let items = resp.list.as_ref().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].email.as_deref().unwrap(), "pub@example.com");
        let uid_list = items[0].userid_list.as_ref().unwrap();
        assert_eq!(uid_list.list, vec!["user1", "user2"]);
    }

    #[test]
    fn test_get_mail_list_response_deserialization() {
        let json = r#"{
            "errcode": 0,
            "errmsg": "ok",
            "next_cursor": "NEXT",
            "has_more": 1,
            "mail_list": [{"mail_id": "ID1"}, {"mail_id": "ID2"}]
        }"#;
        let resp: GetMailListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.errcode, 0);
        assert_eq!(resp.next_cursor.as_deref().unwrap(), "NEXT");
        assert_eq!(resp.has_more, Some(1));
        let mails = resp.mail_list.as_ref().unwrap();
        assert_eq!(mails.len(), 2);
        assert_eq!(mails[0].mail_id.as_deref().unwrap(), "ID1");
    }

    #[test]
    fn test_vip_batch_response_deserialization() {
        let json = r#"{
            "errcode": 0,
            "errmsg": "ok",
            "succ_userid_list": ["zhangsan", "lisi"],
            "fail_userid_list": ["wangwu"]
        }"#;
        let resp: VipBatchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.errcode, 0);
        let succ = resp.succ_userid_list.as_ref().unwrap();
        assert_eq!(succ, &["zhangsan", "lisi"]);
        let fail = resp.fail_userid_list.as_ref().unwrap();
        assert_eq!(fail, &["wangwu"]);
    }

    #[test]
    fn test_vip_list_response_deserialization() {
        let json = r#"{
            "errcode": 0,
            "errmsg": "ok",
            "has_more": true,
            "next_cursor": "GNIJIGEO",
            "userid_list": ["zhangsan", "lisi"]
        }"#;
        let resp: VipListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.errcode, 0);
        assert_eq!(resp.has_more, Some(true));
        let users = resp.userid_list.as_ref().unwrap();
        assert_eq!(users, &["zhangsan", "lisi"]);
    }

    #[test]
    fn test_get_user_option_response_deserialization() {
        let json = r#"{
            "errcode": 0,
            "errmsg": "ok",
            "option": {
                "list": [
                    {"type": 1, "value": "0"},
                    {"type": 2, "value": "1"}
                ]
            }
        }"#;
        let resp: GetUserOptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.errcode, 0);
        let opts = resp.option.as_ref().unwrap();
        assert_eq!(opts.list.len(), 2);
        assert_eq!(opts.list[0].option_type, 1);
        assert_eq!(opts.list[0].value, "0");
        assert_eq!(opts.list[1].option_type, 2);
        assert_eq!(opts.list[1].value, "1");
    }

    #[test]
    fn test_mail_group_detail_deserialization() {
        let json = r#"{
            "errcode": 0,
            "errmsg": "ok",
            "groupid": "g@example.com",
            "groupname": "G1",
            "email_list": {"list": ["a@example.com"]},
            "department_list": {"list": [1]},
            "tag_list": {"list": []},
            "group_list": {"list": []},
            "allow_type": 0
        }"#;
        let resp: MailGroupDetail = serde_json::from_str(json).unwrap();
        assert_eq!(resp.errcode, 0);
        assert_eq!(resp.groupid.as_deref().unwrap(), "g@example.com");
        assert_eq!(resp.allow_type, Some(0));
        let emails = resp.email_list.as_ref().unwrap();
        assert_eq!(emails.list, vec!["a@example.com"]);
    }

    #[test]
    fn test_schedule_reminders_only_serializes_set_fields() {
        let rem = ScheduleReminders {
            is_remind: Some(1),
            remind_before_event_mins: Some(15),
            ..Default::default()
        };
        let json = serde_json::to_value(&rem).unwrap();
        assert_eq!(json["is_remind"], 1);
        assert_eq!(json["remind_before_event_mins"], 15);
        assert!(json.get("is_repeat").is_none());
        assert!(json.get("timezone").is_none());
    }
}
