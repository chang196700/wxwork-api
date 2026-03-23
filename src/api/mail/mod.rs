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

    /// 通用 POST 调用（邮件接口路径以 /cgi-bin/mail/ 开头）
    pub async fn post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    pub async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }

    /// 获取成员邮箱账号 GET /cgi-bin/mail/get_user_email
    pub async fn get_user_email(&self, userid: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/mail/get_user_email", &[("userid", userid)]).await
    }

    /// 查询邮件发送任务 POST /cgi-bin/mail/get_group_send_task
    pub async fn get_group_send_task(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/mail/get_group_send_task", req).await
    }
}
