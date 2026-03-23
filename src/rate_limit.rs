//! 企业微信 API 访问频率限制常量
//! 来源：附录《访问频率限制》

/// 每企业调用单个 API 的最大频率（次/分钟）
pub const RATE_PER_CORP_PER_MINUTE: u32 = 10_000;

/// 每企业调用单个 API 的最大频率（次/小时）
pub const RATE_PER_CORP_PER_HOUR: u32 = 150_000;

/// 每 IP 调用单个 API 的最大频率（次/分钟）
pub const RATE_PER_IP_PER_MINUTE: u32 = 20_000;

/// 每 IP 调用单个 API 的最大频率（次/小时）
pub const RATE_PER_IP_PER_HOUR: u32 = 600_000;

/// 发送应用消息：每应用对同一成员的最大频率（次/分钟）
pub const MSG_PER_MEMBER_PER_MINUTE: u32 = 30;

/// 发送应用消息：每应用对同一成员的最大频率（次/小时）
pub const MSG_PER_MEMBER_PER_HOUR: u32 = 1_000;

/// 获取打卡数据最大频率（次/分钟）
pub const CHECKIN_DATA_PER_MINUTE: u32 = 600;

/// 获取审批数据最大频率（次/分钟）
pub const APPROVAL_DATA_PER_MINUTE: u32 = 600;

/// 获取 jsapi_ticket：每企业每小时最多获取次数
pub const JSAPI_TICKET_PER_HOUR: u32 = 400;

/// 获取 jsapi_ticket：每应用每小时最多获取次数
pub const JSAPI_TICKET_PER_APP_PER_HOUR: u32 = 100;

/// 应用创建群聊最大频率（次/天）
pub const GROUP_CREATE_PER_DAY: u32 = 1_000;

/// 应用变更群聊最大频率（次/小时）
pub const GROUP_MODIFY_PER_HOUR: u32 = 1_000;

/// 上传永久图片最大频率（张/月）
pub const UPLOAD_IMAGE_PER_MONTH: u32 = 3_000;

/// 上传永久图片最大频率（张/天）
pub const UPLOAD_IMAGE_PER_DAY: u32 = 1_000;

/// 创建账号最大频率（账号上限数 × 3 / 月）
/// 实际值 = `corp_user_limit * 3`
pub const ACCOUNT_CREATE_MULTIPLIER: u32 = 3;

/// 创建应用最大频率（最大应用数 × 3 / 月，即 300 × 3 = 900）
pub const APP_CREATE_PER_MONTH: u32 = 900;

/// 设置可信域名最大数量（/月）
pub const TRUSTED_DOMAIN_SET_PER_MONTH: u32 = 20;

/// 按分钟拦截后的封禁时长（秒）
pub const BLOCK_DURATION_MINUTE_LIMIT_SECS: u32 = 60;

/// 按小时拦截后的封禁时长（分钟）
pub const BLOCK_DURATION_HOUR_LIMIT_MINUTES: u32 = 60;

/// 按天拦截后的封禁时长（自然天）
pub const BLOCK_DURATION_DAY_LIMIT_DAYS: u32 = 1;

/// 按月拦截后的封禁时长（天）
pub const BLOCK_DURATION_MONTH_LIMIT_DAYS: u32 = 30;

/// 推送群聊消息最大频率（人次/分钟）
pub const GROUP_MSG_PER_MINUTE: u32 = 20_000;

/// 推送群聊消息最大频率（人次/小时）
pub const GROUP_MSG_PER_HOUR: u32 = 300_000;

/// 每成员在群中收到应用消息最大数（条/分钟）
pub const GROUP_MSG_PER_MEMBER_PER_MINUTE: u32 = 200;

/// 每成员在群中收到应用消息最大数（条/天）
pub const GROUP_MSG_PER_MEMBER_PER_DAY: u32 = 10_000;
