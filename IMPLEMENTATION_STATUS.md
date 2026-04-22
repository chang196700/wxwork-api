# wxwork-api 实现情况分析

> 基于 `H:\wxworkdoc\server-api\` 文档，对照当前代码实现状态的全面分析。

---

## 一、总体概览

| 分类 | 模块数 | 状态 |
|------|--------|------|
| 开发指南 | 1 | ✅ 完整实现 |
| 基础模块 | 10 | 🟡 大部分完整，部分缺失 |
| 办公模块 | 13 | 🟡 核心已实现，细节不足 |
| 连接微信模块 | 6 | 🟡 部分完整，部分仅骨架 |

---

## 二、各模块实现详情

### 开发指南

#### `auth` — 获取 access_token / IP 段

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 获取 access_token | `get_access_token` | ✅ |
| 获取企业微信接口IP段 | `get_api_domain_ip` | ✅ |
| 获取企业微信回调IP段 | `get_callback_ip` | ✅ |

---

### 基础模块

#### `contacts/member` — 通讯录成员管理

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 创建成员 | `create` | ✅ 有类型化 Struct |
| 读取成员 | `get` | ✅ 有类型化 Struct |
| 更新成员 | `update` | ✅ 有类型化 Struct |
| 删除成员 | `delete` | ✅ |
| 批量删除成员 | `batch_delete` | ✅ |
| 获取部门成员 | `list_simple` | ✅ |
| 获取部门成员详情 | `list` | ✅ |
| 获取成员 ID 列表 | `list_id` | ✅ 有类型化 Struct |
| 获取加入企业二维码 | `get_join_qrcode` | ✅ |
| userid 与 openid 互换 | `convert_to_openid` | ✅ |
| 邀请成员 | — | ❌ **缺失** (`POST /cgi-bin/batch/invite`) |
| 手机号获取 userid | — | ❌ **缺失** (`POST /cgi-bin/user/getuserid`) |
| 邮箱获取 userid | — | ❌ **缺失** (`POST /cgi-bin/user/get_userid_by_email`) |
| 登录二次验证 | `get_user_info` | ⚠️ **命名/定位混淆**（见第三节） |

#### `contacts/department` — 部门管理

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 创建部门 | `create` | ✅ 有类型化 Struct |
| 更新部门 | `update` | ✅ 有类型化 Struct |
| 删除部门 | `delete` | ✅ |
| 获取部门列表 | `list` | ✅ |
| 获取单个部门详情 | `get` | ✅ |
| 获取子部门 ID 列表 | `list_simple` | ✅ |

#### `contacts/tag` — 标签管理

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 创建标签 | `create` | ✅ |
| 更新标签名字 | `update` | ✅ |
| 删除标签 | `delete` | ✅ |
| 获取标签成员 | `get` | ✅ |
| 增加标签成员 | `add_users` | ✅ |
| 删除标签成员 | `delete_users` | ✅ |
| 获取标签列表 | `list` | ✅ |

#### `contacts/export_import` — 异步导入/导出

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 导出成员 | `export_simple_user` | ✅ |
| 导出成员详情 | `export_user` | ✅ |
| 导出部门 | `export_department` | ✅ |
| 导出标签成员 | `export_tag_user` | ✅ |
| 获取导出结果 | `get_result`（导出） | ✅ |
| 增量更新成员 | `sync_user` | ✅ |
| 全量覆盖成员 | `replace_user` | ✅ |
| 全量覆盖部门 | `replace_party` | ✅ |
| 获取异步任务结果 | `get_result`（导入） | ✅ |

#### `message/send` — 发送应用消息

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 发送应用消息（text/image/voice/video/file/textcard/news/mpnews/markdown/miniprogram_notice/template_card） | `send` | ✅ 有完整类型化枚举 |
| 撤回应用消息 | `recall` | ✅ |
| 更新模板卡片消息 | `update_template_card` | ✅ |

> ⚠️ `template_card` 内容字段使用 `serde_json::Value`，未做细粒度类型化。

#### `message/group` — 应用发送消息到群聊会话

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 创建群聊会话 | `create_chat` | ✅ |
| 修改群聊会话 | `update_chat` | ✅ |
| 获取群聊会话 | `get_chat` | ✅ |
| 应用推送消息 | `send` | ✅ |

#### `agent` — 应用管理

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 获取应用 | `get` | ✅ |
| 获取应用列表 | `list` | ✅ |
| 设置应用 | `set` | ✅ |
| 设置工作台自定义展示 | `set_workbench_template` | ✅ (使用 Value) |
| 获取自定义菜单 | `get_menu` | ✅ |
| 创建菜单 | `create_menu` | ✅ (使用 Value) |
| 删除菜单 | `delete_menu` | ✅ |

#### `material` — 素材管理

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 上传临时素材 | `upload_temp` | ✅ |
| 获取临时素材 | `get_temp_media_bytes` | ✅ |
| 上传图片 | `upload_image` | ✅ |
| 异步上传临时素材 | — | ❌ **缺失** (`POST /cgi-bin/media/upload_by_url`) |
| 查询异步上传任务结果 | — | ❌ **缺失** (`POST /cgi-bin/media/get_upload_by_url_result`) |
| 获取高清语音素材 | — | ❌ **缺失** (`GET /cgi-bin/media/get/jssdk`) |

#### `identity` — 身份验证（网页授权/Web登录）

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 获取访问用户身份（OAuth2） | `get_user_info` | ✅ |
| 获取访问用户敏感信息 | `get_user_detail` | ✅ |
| 获取企业 jsapi_ticket | `get_jsapi_ticket` | ✅ |
| 获取应用 jsapi_ticket | `get_agent_jsapi_ticket` | ✅ |
| 企业微信 Web 登录（获取用户登录身份） | — | ❌ **缺失**（不同于 OAuth2 的独立接口） |
| 二次验证相关接口 | — | ❌ **缺失** |

#### `account_id` — 账号 ID 转换

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| userid 转换 | `translate_userid` | ✅ (Value) |
| 邮箱/手机号获取 userid | `get_userid_by_email_or_mobile` | ⚠️ **端点与通讯录文档不符**（见第三节） |
| openid 转换 userid | `convert_to_userid` | ✅ 有类型化 Struct |
| tmp_external_userid 转换 | — | ❌ **缺失** (`POST /cgi-bin/idconvert/convert_tmp_external_userid`) |
| open_userid → userid（自建应用对接） | — | ❌ **缺失** (`POST /cgi-bin/batch/openuserid_to_userid`) |
| external_userid 转换（自建对接） | — | ❌ **缺失** (`POST /cgi-bin/externalcontact/from_service_external_userid`) |

#### `security` — 安全管理

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 获取防钓鱼密钥 | `get_anti_phishing_key` | ⚠️ **注释错误**（见第三节） |
| 获取企业微信域名 IP 信息 | `get_domain_ip_list` | ✅ |
| 获取防泄漏设置 | `get_dlp_conf` | ✅ (Value) |
| 获取设备列表 | `get_device_info` | ✅ (Value) |
| 获取截屏录屏权限 | `get_screen_record_conf` | ✅ (Value) |
| 获取成员操作记录 | `get_operation_list` | ✅ (Value) |
| 获取管理端操作日志 | `get_admin_audit_log` | ✅ (Value) |
| 分配高级功能账号 | `assign_quota` | ✅ (Value) |
| 获取高级功能账号列表 | `list_quota` | ✅ (Value) |
| 取消高级功能账号 | `cancel_quota` | ✅ (Value) |
| 设置防泄漏规则 | — | ❌ **缺失** (`POST /cgi-bin/dlp/set_conf`) |

#### `interconnect` — 企业互联

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 获取下级/下游企业 access_token | `get_corp_token` | ✅ (Value) |
| 获取下级/下游企业小程序 session | `transfer_session` | ✅ (Value) |
| 获取应用共享信息 | `get_agent_info` | ✅ (Value) |

> ⚠️ 企业互联的 URL 前缀为 `/cgi-bin/corp/`，而上下游为 `/cgi-bin/corpgroup/corp/`，两者已正确分离，但企业互联文档中的更多接口（互联企业通讯录管理等）尚未实现。

#### `upstream` — 上下游

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 获取下游企业 access_token | `get_corp_token` | ✅ (Value) |
| 获取下游小程序 session | `transfer_session` | ✅ (Value) |
| 获取应用共享信息 | `get_agent_info` | ✅ (Value) |
| 上下游通讯录管理（多接口） | 部分实现 | 🟡 文档较多，代码仅 11 个接口 |

#### `invoice` — 电子发票

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 查询电子发票 | `get_invoice_info` | ✅ (Value) |
| 批量查询电子发票 | `get_invoice_info_batch` | ✅ (Value) |
| 更新发票状态 | `update_invoice_status` | ✅ (Value) |
| 批量更新发票状态 | — | ❌ **缺失** (`POST /cgi-bin/card/invoice/reimburse/updateinvoicestatusv2`) |

#### `data_intel` — 数据与智能专区

> 实现了通用 `post/get` 方法，以及 2 个与 `checkin` 重复的打卡接口。  
> 文档中数据与智能专区实际是独立运行环境的 SDK 接口，HTTP 层面接口有限，现有实现基本满足需求，但存在与 `checkin` 的重复（见第三节）。

---

### 办公模块

#### `approval` — 审批

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 获取审批模板详情 | `get_template_detail` | ✅ (Value) |
| 创建审批模板 | `create_template` | ✅ (Value) |
| 更新审批模板 | `update_template` | ✅ |
| 提交审批申请 | `apply_event` | ✅ 有类型化 Struct |
| 批量获取审批单号 | `get_approval_info` | ✅ 有类型化 Struct |
| 获取审批申请详情 | `get_approval_detail` | ✅ (Value) |
| 获取审批数据（旧） | `get_approval_data` | ✅ (Value) |
| 获取企业假期管理配置 | `get_corp_vacation_conf` | ✅ (Value) |
| 获取成员假期余额 | `get_user_vacation_quota` | ✅ (Value) |
| 修改成员假期余额 | `set_user_vacation_quota` | ✅ (Value) |

#### `calendar` — 日程

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 创建日历 | `add_calendar` | ✅ (Value) |
| 更新日历 | `update_calendar` | ✅ |
| 删除日历 | `del_calendar` | ✅ |
| 获取日历详情 | `get_calendar` | ✅ (Value) |
| 创建日程 | `add_schedule` | ✅ (Value) |
| 更新日程 | `update_schedule` | ✅ |
| 删除日程 | `del_schedule` | ✅ |
| 获取日程详情 | `get_schedule` | ✅ (Value) |
| 取消日程 | `cancel_schedule` | ✅ |

#### `checkin` — 打卡

实现了 12 个接口，覆盖打卡规则查询、打卡记录获取、日报/月报、排班、补卡、人脸录入等。整体较完整。

#### `meeting` — 会议

实现了 17 个接口，覆盖预约会议基础/高级管理、会中控制、录制管理、会议统计、Webinar、电话入会等。整体较完整。

#### `meeting_room` — 会议室

实现了 6 个接口（含通用 post/get），覆盖会议室列表、预定、取消等基础功能。文档中更多精细接口（会议室权限、设备管理等）尚未类型化。

#### `docs` — 文档

实现了 13 个接口，覆盖文档创建/重命名/删除、权限、收集表、智能表格等。整体较完整。

#### `drive` — 微盘

实现了 13 个接口，覆盖空间、文件、成员权限等。整体较完整。

#### `live` — 直播

实现了 9 个接口，同时暴露了通用 `post` 方法（设计不一致，见第三节）。

#### `mail` — 邮件

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 发送邮件（普通/日程/会议） | `compose_send` | ✅ 完整类型化 Struct |
| 获取成员邮箱账号 | `get_user_email` | ✅ 有类型化 Struct |
| 禁用/启用邮箱账号 | `act_email` | ✅ 有类型化 Struct |
| 获取邮件未读数 | `get_new_count` | ✅ 有类型化 Struct |
| 查询应用邮箱账号及别名 | `get_app_email_alias` | ✅ 有类型化 Struct |
| 更新应用邮箱账号 | `update_app_email_alias` | ✅ |
| 创建公共邮箱 | `create_public_mail` | ✅ 完整类型化 Struct |
| 获取公共邮箱详情 | `get_public_mail` | ✅ 完整类型化 Struct |
| 更新公共邮箱 | `update_public_mail` | ✅ 完整类型化 Struct |
| 删除公共邮箱 | `delete_public_mail` | ✅ |
| 模糊搜索公共邮箱 | `search_public_mail` | ✅ 有类型化 Struct |
| 获取客户端专用密码列表 | `get_public_mail_auth_codes` | ✅ 有类型化 Struct |
| 删除客户端专用密码 | `delete_public_mail_auth_code` | ✅ |
| 创建邮件群组 | `create_mail_group` | ✅ 完整类型化 Struct |
| 获取邮件群组详情 | `get_mail_group` | ✅ 完整类型化 Struct |
| 更新邮件群组 | `update_mail_group` | ✅ 完整类型化 Struct |
| 删除邮件群组 | `delete_mail_group` | ✅ |
| 模糊搜索邮件群组 | `search_mail_group` | ✅ 有类型化 Struct |
| 获取收件箱邮件列表 | `get_mail_list` | ✅ 有类型化 Struct |
| 获取邮件内容（eml） | `read_mail` | ✅ 有类型化 Struct |
| 获取用户功能属性 | `get_user_option` | ✅ 完整类型化 Struct |
| 更改用户功能属性 | `update_user_option` | ✅ 完整类型化 Struct |
| 分配高级功能账号 | `vip_batch_add` | ✅ 有类型化 Struct |
| 取消高级功能账号 | `vip_batch_del` | ✅ 有类型化 Struct |
| 获取高级功能账号列表 | `vip_list` | ✅ 有类型化 Struct |
| 查询邮件发送任务 | `get_group_send_task` | 🟡 Value（文档未详述） |

#### `report` — 汇报

实现了 3 个接口（获取汇报记录列表/详情/统计数据），覆盖文档中核心接口。

#### `hr` — 人事助手

实现了 3 个接口（获取花名册字段、获取员工信息、更新员工信息），与文档核心接口对应，均使用 `serde_json::Value`。

#### `phone` — 公费电话

仅实现 1 个接口（获取拨打记录），与文档一致。

#### `alert` — 紧急通知应用

仅实现 1 个接口（获取紧急通知成员列表），与文档接口对应。

#### `advanced_feat` — 高级功能（审批流程引擎）

| 文档接口 | 实现函数 | 状态 |
|----------|----------|------|
| 批量获取申请单 ID | — | ❌ 仅通用 `post/get` |
| 获取申请单详细信息 | — | ❌ 仅通用 `post/get` |
| 设置审批单审批信息 | — | ❌ 仅通用 `post/get` |
| 成员申请提交/终止回调（接收端） | — | ❌ |

---

### 连接微信模块

#### `excontact` — 客户联系

实现了 24 个接口，覆盖客户列表/详情、客户备注、客户群、联系我等。整体较完整。

#### `wechat_cs` — 微信客服

实现了 15 个接口，覆盖客服账号管理、接待人员管理、会话分配、接待消息等。整体较完整。

#### `chat_archive` — 会话内容存档

实现了 4 个接口（获取开启成员列表、查询同意情况、检查群聊同意情况、获取产品图册）。  
> ⚠️ 会话内容存档核心（拉取会话数据）依赖企业微信 C SDK，不属于 HTTP API，已在注释中说明。

#### `payment` — 企业支付

实现了 7 个接口（付款、查询记录、红包、对外收款等）。与文档基本对应。

#### `school` — 家校沟通

实现了 5 个接口，覆盖学校 token、学生/家长详情、发送通知等基础接口。文档中更多精细接口尚未实现。

#### `miniprogram_pay` — 小程序对外收款

实现了 5 个接口（下单、查询、退款、查退款、获取通知数据）。与文档基本对应。

---

## 三、实现与文档不符 / 已知问题（已全部修复）

### 3.1 ✅ 注释错误（已修复）

**文件：** `src/api/security/mod.rs`

`get_anti_phishing_key` 的注释已从"获取企业微信域名 IP 信息"改为"获取防钓鱼密钥"。

---

### 3.2 ✅ 接口定位混淆（已修复）

**文件：** `src/api/contacts/member.rs`

- 已从 `MemberApi` 移除重复的 `get_user_info`（`/cgi-bin/user/getuserinfo`），该接口属于身份验证模块，已由 `identity.rs` 正确实现
- 同步移除了仅被该方法使用的 `UserInfoResponse` 类型

---

### 3.3 ✅ 缺失的通讯录 userid 查询接口（已修复）

**文件：** `src/api/contacts/member.rs`，`src/api/account_id/mod.rs`

- 已在 `MemberApi` 中新增 `get_userid_by_mobile` (`POST /cgi-bin/user/getuserid`)
- 已在 `MemberApi` 中新增 `get_userid_by_email` (`POST /cgi-bin/user/get_userid_by_email`)
- 已从 `AccountIdApi` 移除命名误导、端点无文档依据的 `get_userid_by_email_or_mobile`

---

### 3.4 ✅ data_intel 与 checkin 接口重复（已修复）

**文件：** `src/api/data_intel/mod.rs`

已移除 `data_intel` 中重复的 `get_checkin_data`、`get_checkin_day_data`、`get_checkin_month_data` 以及通用 `post()`/`get()` 方法。打卡接口应统一通过 `checkin()` 调用。

---

### 3.5 ✅ 暴露通用方法，设计不一致（已修复）

**文件：** `src/api/live/mod.rs`、`src/api/meeting_room/mod.rs`、`src/api/advanced_feat/mod.rs`

已从上述模块移除绕过类型约束的通用 `post()`/`get()` 方法，与 `approval`、`checkin` 等模块风格一致。

---

### 3.6 仍缺失的接口

| 模块 | 缺失接口 | 文档端点 |
|------|----------|----------|
| `contacts/member` | 邀请成员 | `POST /cgi-bin/batch/invite` |
| `material` | 异步上传临时素材 | `POST /cgi-bin/media/upload_by_url` |
| `material` | 查询异步上传结果 | `POST /cgi-bin/media/get_upload_by_url_result` |
| `material` | 获取高清语音素材 | `GET /cgi-bin/media/get/jssdk` |
| `invoice` | 批量更新发票状态 | `POST /cgi-bin/card/invoice/reimburse/updateinvoicestatusv2` |
| `account_id` | tmp_external_userid 转换 | `POST /cgi-bin/idconvert/convert_tmp_external_userid` |
| `account_id` | open_userid → userid | `POST /cgi-bin/batch/openuserid_to_userid` |
| `account_id` | external_userid 转换（自建对接） | `POST /cgi-bin/externalcontact/from_service_external_userid` |
| `identity` | 企业微信 Web 登录 | `POST /cgi-bin/auth/getuserinfo` |
| `security` | 设置防泄漏规则 | `POST /cgi-bin/dlp/set_conf` |
| `advanced_feat` | 批量获取申请单 ID | `POST /cgi-bin/oa/approval/batch_get_application` |
| `advanced_feat` | 获取申请单详细信息 | `POST /cgi-bin/oa/approval/get_application_detail` |
| `advanced_feat` | 设置审批单审批信息 | `POST /cgi-bin/oa/approval/set_approval_info` |

---

## 四、类型化程度评估

> 使用 `serde_json::Value` 而非具体 Struct 的模块，对调用方缺少编译期类型安全保障。

| 类型化程度 | 模块 |
|-----------|------|
| **完全类型化**（请求/响应均有 Struct） | `contacts/*`, `message/send`, `message/group`, `agent`, `approval`（部分）, `identity`, `material`, `mail` |
| **部分类型化**（请求或响应之一有 Struct） | `auth`, `approval`, `meeting`, `checkin`, `calendar`, `excontact`, `wechat_cs`, `security`, `upstream`, `invoice` |
| **通用 JSON**（全部使用 Value） | `alert`, `data_intel`（空壳）, `hr`, `phone`, `report`, `school`, `interconnect`, `live`（混合）, `meeting_room`（混合）, `advanced_feat`（空壳） |
