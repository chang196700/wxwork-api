#![allow(unused_imports)]
//! # wxwork-api
//!
//! 企业微信（WeCom）服务端 API SDK，基于 Rust + tokio 异步运行时构建。
//!
//! ## 特性
//!
//! - 支持 HTTP / HTTPS / SOCKS5 代理（通过 `reqwest` socks feature）
//! - access_token 自动缓存与刷新（提前 5 分钟刷新，多协程安全）
//! - 覆盖企业微信所有服务端 API 分类（711+ 接口）
//! - lib + binary 双模式
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use wxwork_api::{WxWorkClient, WxWorkConfig, ProxyConfig};
//!
//! #[tokio::main]
//! async fn main() -> wxwork_api::error::Result<()> {
//!     let config = WxWorkConfig::new("your_corpid", "your_secret")
//!         .with_proxy(ProxyConfig::socks5("socks5://127.0.0.1:1080"));
//!
//!     let client = WxWorkClient::new(config)?;
//!
//!     // 获取 access_token（自动缓存）
//!     let token = client.access_token().await?;
//!     println!("token: {}", token);
//!
//!     // 发送文本消息
//!     use wxwork_api::api::message::send::SendMessageRequest;
//!     let req = SendMessageRequest::text(1000002, "@all", "Hello from Rust!");
//!     let resp = client.message().send().send(&req).await?;
//!     println!("msgid: {:?}", resp.msgid);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod config;
pub mod crypto;
pub mod error;
pub mod error_codes;
pub mod rate_limit;
pub mod token;
pub mod types;

pub use client::WxWorkClient;
pub use config::{ProxyConfig, WxWorkConfig};
pub use error::{Result, WxWorkError};

use api::{
    account_id::AccountIdApi,
    advanced_feat::AdvancedFeatApi,
    agent::AgentApi,
    alert::AlertApi,
    approval::ApprovalApi,
    auth::AuthApi,
    calendar::CalendarApi,
    chat_archive::ChatArchiveApi,
    checkin::CheckinApi,
    contacts::ContactsApi,
    data_intel::DataIntelApi,
    docs::DocsApi,
    drive::DriveApi,
    excontact::ExContactApi,
    hr::HrApi,
    identity::IdentityApi,
    interconnect::InterconnectApi,
    invoice::InvoiceApi,
    live::LiveApi,
    mail::MailApi,
    material::MaterialApi,
    meeting::MeetingApi,
    meeting_room::MeetingRoomApi,
    message::MessageApi,
    miniprogram_pay::MiniprogramPayApi,
    payment::PaymentApi,
    phone::PhoneApi,
    report::ReportApi,
    school::SchoolApi,
    security::SecurityApi,
    upstream::UpstreamApi,
    wechat_cs::WechatCsApi,
};

/// WxWorkClient API 模块访问扩展
impl WxWorkClient {
    // ====== 开发指南 ======

    pub fn auth(&self) -> AuthApi<'_> {
        AuthApi::new(self)
    }

    // ====== 基础模块 ======

    pub fn contacts(&self) -> ContactsApi<'_> {
        ContactsApi::new(self)
    }

    pub fn message(&self) -> MessageApi<'_> {
        MessageApi::new(self)
    }

    pub fn agent(&self) -> AgentApi<'_> {
        AgentApi::new(self)
    }

    pub fn material(&self) -> MaterialApi<'_> {
        MaterialApi::new(self)
    }

    pub fn identity(&self) -> IdentityApi<'_> {
        IdentityApi::new(self)
    }

    pub fn security(&self) -> SecurityApi<'_> {
        SecurityApi::new(self)
    }

    pub fn account_id(&self) -> AccountIdApi<'_> {
        AccountIdApi::new(self)
    }

    pub fn upstream(&self) -> UpstreamApi<'_> {
        UpstreamApi::new(self)
    }

    pub fn interconnect(&self) -> InterconnectApi<'_> {
        InterconnectApi::new(self)
    }

    pub fn invoice(&self) -> InvoiceApi<'_> {
        InvoiceApi::new(self)
    }

    pub fn data_intel(&self) -> DataIntelApi<'_> {
        DataIntelApi::new(self)
    }

    // ====== 办公模块 ======

    pub fn meeting(&self) -> MeetingApi<'_> {
        MeetingApi::new(self)
    }

    pub fn meeting_room(&self) -> MeetingRoomApi<'_> {
        MeetingRoomApi::new(self)
    }

    pub fn docs(&self) -> DocsApi<'_> {
        DocsApi::new(self)
    }

    pub fn drive(&self) -> DriveApi<'_> {
        DriveApi::new(self)
    }

    pub fn mail(&self) -> MailApi<'_> {
        MailApi::new(self)
    }

    pub fn calendar(&self) -> CalendarApi<'_> {
        CalendarApi::new(self)
    }

    pub fn approval(&self) -> ApprovalApi<'_> {
        ApprovalApi::new(self)
    }

    pub fn checkin(&self) -> CheckinApi<'_> {
        CheckinApi::new(self)
    }

    pub fn live(&self) -> LiveApi<'_> {
        LiveApi::new(self)
    }

    pub fn report(&self) -> ReportApi<'_> {
        ReportApi::new(self)
    }

    pub fn alert(&self) -> AlertApi<'_> {
        AlertApi::new(self)
    }

    pub fn hr(&self) -> HrApi<'_> {
        HrApi::new(self)
    }

    pub fn phone(&self) -> PhoneApi<'_> {
        PhoneApi::new(self)
    }

    pub fn advanced_feat(&self) -> AdvancedFeatApi<'_> {
        AdvancedFeatApi::new(self)
    }

    // ====== 连接微信模块 ======

    pub fn excontact(&self) -> ExContactApi<'_> {
        ExContactApi::new(self)
    }

    pub fn wechat_cs(&self) -> WechatCsApi<'_> {
        WechatCsApi::new(self)
    }

    pub fn chat_archive(&self) -> ChatArchiveApi<'_> {
        ChatArchiveApi::new(self)
    }

    pub fn payment(&self) -> PaymentApi<'_> {
        PaymentApi::new(self)
    }

    pub fn school(&self) -> SchoolApi<'_> {
        SchoolApi::new(self)
    }

    pub fn miniprogram_pay(&self) -> MiniprogramPayApi<'_> {
        MiniprogramPayApi::new(self)
    }
}
