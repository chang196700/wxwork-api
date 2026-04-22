use serde::Deserialize;
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 安全管理 API
pub struct SecurityApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> SecurityApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 获取防钓鱼密钥 GET /cgi-bin/security/get_anti_phishing_key
    pub async fn get_anti_phishing_key(&self) -> Result<AntiPhishingKeyResponse> {
        self.client.get("/cgi-bin/security/get_anti_phishing_key", &[]).await
    }

    /// 获取企业微信域名 IP 信息 GET /cgi-bin/get_domain_iplist
    pub async fn get_domain_ip_list(&self) -> Result<IpListResponse> {
        self.client.get("/cgi-bin/get_domain_iplist", &[]).await
    }

    /// 文件防泄漏 - 获取防泄漏设置 GET /cgi-bin/dlp/get_conf
    pub async fn get_dlp_conf(&self) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/dlp/get_conf", &[]).await
    }

    /// 设备管理 - 获取设备列表 POST /cgi-bin/mdm/get_device_info
    pub async fn get_device_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/mdm/get_device_info", req).await
    }

    /// 截屏录屏管理 - 获取截屏录屏权限 GET /cgi-bin/security/get_screen_record
    pub async fn get_screen_record_conf(&self) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/security/get_screen_record", &[]).await
    }

    /// 获取成员操作记录 POST /cgi-bin/security/operationlist
    pub async fn get_operation_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/security/operationlist", req).await
    }

    /// 获取管理端操作日志 POST /cgi-bin/adminaudit/getadminauditlog
    pub async fn get_admin_audit_log(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/adminaudit/getadminauditlog", req).await
    }

    /// 分配高级功能账号 POST /cgi-bin/security/quota/assign
    pub async fn assign_quota(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/security/quota/assign", req).await
    }

    /// 获取高级功能账号列表 POST /cgi-bin/security/quota/list
    pub async fn list_quota(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/security/quota/list", req).await
    }

    /// 取消高级功能账号 POST /cgi-bin/security/quota/cancel
    pub async fn cancel_quota(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/security/quota/cancel", req).await
    }
}

#[derive(Debug, Deserialize)]
pub struct AntiPhishingKeyResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IpListResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub ip_list: Option<Vec<String>>,
}
