# wxwork-api TODO 列表

> 基于 `IMPLEMENTATION_STATUS.md` 分析结果，按优先级排列的待办事项。

---

## P0 — 错误修复（影响正确性）

### TODO-01：修复 `security.rs` 中 `get_anti_phishing_key` 注释错误

**文件：** `src/api/security/mod.rs`

当前注释为"获取企业微信域名 IP 信息"，应改为"获取防钓鱼密钥"。

```rust
// 修改前
/// 获取企业微信域名 IP 信息 GET /cgi-bin/security/get_anti_phishing_key

// 修改后
/// 获取防钓鱼密钥 GET /cgi-bin/security/get_anti_phishing_key
```

---

### TODO-02：消除 `member.rs` 与 `identity.rs` 的重复接口

**文件：** `src/api/contacts/member.rs`

`member.rs` 中的 `get_user_info` 与 `identity.rs` 中的 `get_user_info` 调用同一端点 `GET /cgi-bin/user/getuserinfo`，且 `member.rs` 的注释"手机号随机串（登录二次验证）"描述错误。

- 从 `member.rs` 中**删除** `get_user_info` 函数（该功能属于身份验证，已由 `identity.rs` 承担）
- 如果需要保留"登录二次验证"入口，应独立实现专属接口并放入 `identity.rs`

---

### TODO-03：修复 `data_intel.rs` 中重复的打卡接口

**文件：** `src/api/data_intel/mod.rs`

`get_checkin_data` 和 `get_checkin_day_data` 已在 `checkin.rs` 中实现，`data_intel.rs` 中的重复实现应删除，调用方应使用 `client.checkin()` 访问。

---

## P1 — 缺失接口补全（通讯录/素材等核心模块）

### TODO-04：在 `member.rs` 中添加邀请成员接口

**端点：** `POST /cgi-bin/batch/invite`

```rust
pub async fn invite(&self, req: &InviteMemberRequest) -> Result<InviteMemberResponse>

pub struct InviteMemberRequest {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub touser: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub toparty: Vec<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub totag: Vec<u32>,
}

pub struct InviteMemberResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub invaliduser: Option<Vec<String>>,
    pub invalidparty: Option<Vec<u64>>,
    pub invalidtag: Option<Vec<u32>>,
}
```

---

### TODO-05：在 `member.rs` 中添加手机号获取 userid 接口

**端点：** `POST /cgi-bin/user/getuserid`

```rust
pub async fn get_userid_by_mobile(&self, mobile: &str) -> Result<UseridResponse>
```

---

### TODO-06：在 `member.rs` 中添加邮箱获取 userid 接口

**端点：** `POST /cgi-bin/user/get_userid_by_email`

```rust
pub async fn get_userid_by_email(
    &self,
    email: &str,
    email_type: Option<u8>,
) -> Result<UseridResponse>
```

---

### TODO-07：在 `material.rs` 中添加异步上传素材相关接口

**端点：**
- `POST /cgi-bin/media/upload_by_url`（生成异步上传任务）
- `POST /cgi-bin/media/get_upload_by_url_result`（查询任务结果）

```rust
pub async fn upload_by_url(&self, req: &UploadByUrlRequest) -> Result<JobIdResponse>
pub async fn get_upload_by_url_result(&self, jobid: &str) -> Result<UploadByUrlResultResponse>
```

---

### TODO-08：在 `material.rs` 中添加获取高清语音素材接口

**端点：** `GET /cgi-bin/media/get/jssdk`

```rust
pub async fn get_jssdk_media_bytes(&self, media_id: &str) -> Result<bytes::Bytes>
```

---

### TODO-09：在 `invoice.rs` 中添加批量更新发票状态接口

**端点：** `POST /cgi-bin/card/invoice/reimburse/updateinvoicestatusv2`

```rust
pub async fn update_invoice_status_batch(&self, req: &serde_json::Value) -> Result<BaseResponse>
```

---

## P2 — 账号 ID 模块补全

### TODO-10：在 `account_id.rs` 中添加 `tmp_external_userid` 转换接口

**端点：** `POST /cgi-bin/idconvert/convert_tmp_external_userid`

文档参数：`business_type`（uint32）、`user_type`（uint32）、`tmp_external_userid_list`（string[]，最多100个）

---

### TODO-11：在 `account_id.rs` 中添加 open_userid → userid 转换接口

**端点：** `POST /cgi-bin/batch/openuserid_to_userid`

文档参数：`open_userid_list`（string[]）、`source_agentid`（uint32）

---

### TODO-12：在 `account_id.rs` 中添加 external_userid 转换（自建应用对接）

**端点：** `POST /cgi-bin/externalcontact/from_service_external_userid`

文档参数：`external_userid`（string）、`source_agentid`（uint32）

---

## P2 — 身份验证模块补全

### TODO-13：在 `identity.rs` 中添加企业微信 Web 登录接口

企业微信 Web 登录（`/cgi-bin/auth/getuserinfo`）与网页授权登录的 `/cgi-bin/user/getuserinfo` 是不同接口，需要分别实现。

参考文档：`身份验证/企业微信Web登录/获取用户登录身份.md`

---

### TODO-14：在 `identity.rs` 中补充二次验证接口

参考文档：`身份验证/二次验证/`

包括：
- 获取用户二次验证信息
- 使用二次验证

---

## P3 — 安全管理补全

### TODO-15：在 `security.rs` 中添加设置防泄漏规则接口

**端点：** `POST /cgi-bin/dlp/set_conf`

---

## P3 — 邮件模块完整实现

### ~~TODO-16：在 `mail.rs` 中实现完整邮件接口~~ ✅ **已完成**

`mail.rs` 已全面实现，覆盖 25 个接口，全部使用类型化 Struct。

---

## P3 — 高级功能（审批流程引擎）补全

### TODO-17：在 `advanced_feat.rs` 中实现具体接口

当前 `advanced_feat.rs` 只有通用 `post/get` 方法，需要添加：

- **批量获取申请单 ID**（`POST /cgi-bin/oa/approval/batch_get_application`）
- **获取申请单详细信息**（`POST /cgi-bin/oa/approval/get_application_detail`）
- **设置审批单审批信息**（`POST /cgi-bin/oa/approval/set_approval_info`）

---

## P4 — 代码设计改进

### TODO-18：移除 `live.rs` 中暴露的通用 `post` 方法

`live.rs` 中的 `pub async fn post(...)` 通用方法应删除，改为全部使用具体业务函数，与其他模块设计保持一致。

同类问题模块：`mail.rs`, `meeting_room.rs`（已有 `post/get`）。

---

### TODO-19：为使用 `serde_json::Value` 的高频接口补充类型化 Struct

以下接口参数/返回值应替换为类型化 Struct，提升编译期安全性和文档可读性（按优先级）：

1. `approval.get_template_detail` / `apply_event` 返回值
2. `checkin.*` 所有返回值
3. `calendar.*` 请求参数
4. `meeting.*` 请求参数
5. `excontact.*` 主要接口请求/响应
6. `drive.*` 请求参数
7. `docs.*` 请求参数
8. `security.*` 请求参数
9. `upstream.*` 请求参数

---

### TODO-20：统一 `contacts/member.rs` 中 `UseridResponse` 类型

`get_userid_by_mobile`、`get_userid_by_email`（TODO-05/06）以及 `邀请成员`（TODO-04）中的 userid 返回值应共用一个 `UseridResponse` 类型，避免重复定义。

---

## 汇总统计

| 优先级 | 数量 | 类别 |
|--------|------|------|
| P0 | 3 | 错误修复 |
| P1 | 6 | 缺失接口（核心） |
| P2 | 5 | 账号ID + 身份验证补全 |
| P3 | 3 | 安全/邮件/高级功能补全 |
| P4 | 3 | 代码设计改进 |
| **合计** | **20** | |
