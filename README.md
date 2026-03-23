# wxwork-api

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

企业微信（WeCom / WxWork）服务端 API SDK，基于 **Rust + Tokio** 异步运行时构建。

## 特性

- 🔑 **access_token 自动缓存与刷新** — 提前 5 分钟刷新，`Arc<RwLock>` 多协程安全，双重检查锁定
- 🌐 **代理支持** — HTTP / HTTPS / SOCKS5（含认证），兼容所有网络环境
- 📦 **全量 API 覆盖** — 覆盖企业微信全部服务端 API 分类（通讯录、消息、审批、打卡、会议、文档、外部联系人等 31 个模块）
- 🔒 **回调加解密** — 按官方附录实现 AES-256-CBC + SHA1 签名验证（`WxWorkCrypto`）
- 📋 **1578 条全局错误码** — `errcode_description()` 一键查询中文说明
- ⚡ **频率限制常量** — 内置 `rate_limit` 模块，方便业务侧限流
- 🖥️ **CLI 工具** — 随库附带 `wxwork` 命令行工具，可直接调试接口

## 快速开始

### 添加依赖

```toml
[dependencies]
wxwork-api = { git = "https://github.com/chang196700/wxwork-api" }
tokio = { version = "1", features = ["full"] }
```

### 基本用法

```rust
use wxwork_api::{WxWorkClient, WxWorkConfig, ProxyConfig};
use wxwork_api::api::message::send::SendMessageRequest;

#[tokio::main]
async fn main() -> wxwork_api::error::Result<()> {
    // 构建客户端（可选代理）
    let config = WxWorkConfig::new("your_corpid", "your_corpsecret")
        .with_proxy(ProxyConfig::socks5("socks5://127.0.0.1:1080"))
        .with_timeout(30);

    let client = WxWorkClient::new(config)?;

    // access_token 自动获取并缓存
    let token = client.access_token().await?;
    println!("token: {}", token);

    // 发送文本消息
    let req = SendMessageRequest::text(1000002, "@all", "Hello from Rust! 🦀");
    let resp = client.message().send().send(&req).await?;
    println!("msgid: {:?}", resp.msgid);

    // 获取部门列表
    let depts = client.contacts().department().list(None).await?;
    for d in &depts.department {
        println!("dept: {:?}", d.name);
    }

    Ok(())
}
```

### 回调消息加解密

```rust
use wxwork_api::crypto::WxWorkCrypto;

let crypto = WxWorkCrypto::new(
    "YourToken",
    "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C", // EncodingAESKey（43位）
    "wx5823bf96d3bd56c7",                              // corpid（企业应用场景）
)?;

// 验证 URL 回调
let echo = crypto.verify_url(msg_signature, timestamp, nonce, echostr)?;

// 解密推送消息
let xml = crypto.decrypt_message(msg_signature, timestamp, nonce, &post_body)?;

// 加密被动响应
let reply_xml = crypto.encrypt_message(&reply_xml, timestamp, nonce)?;
```

### 错误码查询

```rust
use wxwork_api::error_codes::errcode_description;
use wxwork_api::WxWorkError;

// 直接查询
println!("{:?}", errcode_description(48002));
// => Some("API接口无权限调用")

// 从错误对象获取
let err = WxWorkError::api(40001, "invalid secret");
println!("{:?}", err.description());
// => Some("不合法的secret参数")
```

## CLI 工具

```bash
# 编译
cargo build --release

# 获取 access_token
wxwork --corp-id wxXXX --corp-secret YYY get-token

# 发送文本消息
wxwork --corp-id wxXXX --corp-secret YYY send-text --agent-id 1000002 --to-user "@all" --content "Hello"

# 发送 Markdown 消息
wxwork --corp-id wxXXX --corp-secret YYY send-markdown --agent-id 1000002 --to-user "zhangsan" --content "# 标题"

# 获取成员信息
wxwork --corp-id wxXXX --corp-secret YYY get-user --userid zhangsan

# 获取部门列表
wxwork --corp-id wxXXX --corp-secret YYY list-dept

# 也支持环境变量
export WXWORK_CORP_ID=wxXXX
export WXWORK_CORP_SECRET=YYY
export WXWORK_PROXY=socks5://127.0.0.1:1080
wxwork get-token
```

## API 模块

| 模块 | 方法 | 说明 |
|------|------|------|
| `client.auth()` | `get_access_token` / `get_api_domain_ip` / `get_callback_ip` | 开发基础 |
| `client.contacts().member()` | CRUD + 列表 + userid/openid 转换 | 成员管理 |
| `client.contacts().department()` | CRUD + 列表 | 部门管理 |
| `client.contacts().tag()` | CRUD + 成员管理 | 标签管理 |
| `client.message().send()` | `send` / `recall` / `update_template_card` | 应用消息 |
| `client.message().group()` | 群聊会话管理 + 发消息 | 群机器人 |
| `client.agent()` | 应用详情 + 菜单管理 | 应用管理 |
| `client.material()` | 上传/获取/删除素材 | 素材管理 |
| `client.approval()` | 审批流程 | 审批 OA |
| `client.checkin()` | 打卡数据查询 | 打卡 |
| `client.meeting()` | 会议预约 / 详情 / 列表 | 会议 |
| `client.meeting_room()` | 会议室管理 | 会议室 |
| `client.docs()` | 文档创建 / 获取 | 文档 |
| `client.drive()` | 微盘文件管理 | 微盘 |
| `client.mail()` | 邮件发送 | 邮件 |
| `client.calendar()` | 日历 / 日程管理 | 日历 |
| `client.excontact()` | 客户联系 | 外部联系人 |
| `client.wechat_cs()` | 微信客服 | 客服 |
| `client.school()` | 家校沟通 | 教育 |
| `client.payment()` / `client.miniprogram_pay()` | 对公付款 / 小程序支付 | 支付 |
| ... | | 共 31 个模块 |

## 代理配置

```rust
use wxwork_api::ProxyConfig;

// HTTP 代理
ProxyConfig::http("http://proxy.example.com:8080")

// HTTPS 代理
ProxyConfig::https("https://proxy.example.com:8080")

// SOCKS5 代理
ProxyConfig::socks5("socks5://127.0.0.1:1080")

// 带认证的代理
ProxyConfig::with_auth("socks5://proxy.example.com:1080", "username", "password")
```

## 错误处理

```rust
use wxwork_api::{WxWorkError, error_codes::errcode_description};

match client.contacts().member().get("unknown_user").await {
    Ok(member) => println!("{:?}", member.name),
    Err(WxWorkError::ApiError { errcode, errmsg }) => {
        let desc = errcode_description(errcode).unwrap_or("未知错误");
        eprintln!("API错误 {errcode}（{desc}）: {errmsg}");
    }
    Err(e) if e.is_token_expired() => {
        // token 已过期，可调用 client.token_mgr.refresh_token() 强制刷新
        eprintln!("Token 已过期: {e}");
    }
    Err(e) => eprintln!("其他错误: {e}"),
}
```

## 测试

```bash
# 运行全部测试（76 个，含集成测试，使用 wiremock 模拟服务器，无需真实密钥）
cargo test

# 只运行单元测试
cargo test --lib

# 运行特定模块测试
cargo test crypto
cargo test contacts
```

## 项目结构

```
src/
├── lib.rs              # 库入口，所有 API accessor
├── main.rs             # CLI 入口 (wxwork 命令)
├── client.rs           # WxWorkClient，HTTP get/post/upload 封装
├── config.rs           # WxWorkConfig，ProxyConfig
├── token.rs            # TokenManager，自动缓存刷新
├── error.rs            # WxWorkError，Result<T>
├── error_codes.rs      # 1578 条全局错误码查询表
├── crypto.rs           # WxWorkCrypto，回调消息 AES-CBC 加解密
├── rate_limit.rs       # 访问频率限制常量
├── types/
│   └── common.rs       # 通用响应类型
└── api/
    ├── macros.rs        # api_get! / api_post! 宏
    ├── auth.rs          # 开发指南 API
    ├── contacts/        # 通讯录（成员/部门/标签/导入导出）
    ├── message/         # 消息（应用消息/群聊）
    ├── agent/           # 应用管理
    ├── approval/        # 审批
    ├── checkin/         # 打卡
    ├── meeting/         # 会议
    ├── excontact/       # 外部联系人
    └── ...              # 共 31 个模块
tests/
├── common/mod.rs        # wiremock 测试辅助
├── test_client.rs       # 客户端集成测试
├── test_auth_api.rs     # Auth API 测试
├── test_contacts_api.rs # 通讯录 API 测试
└── test_message_api.rs  # 消息 API 测试
```

## 参考文档

- [企业微信开发文档](https://developer.work.weixin.qq.com/document/path/90556)
- [全局错误码](https://developer.work.weixin.qq.com/document/path/90313)
- [消息加解密方案](https://developer.work.weixin.qq.com/document/path/90968)

## License

MIT
