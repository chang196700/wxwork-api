use serde::{Deserialize, Serialize};
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 连接微信 - 客户联系 API
pub struct ExContactApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> ExContactApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    // ====== 客户管理 ======

    /// 获取客户列表 GET /cgi-bin/externalcontact/list
    pub async fn list_external_contact(&self, userid: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/externalcontact/list", &[("userid", userid)]).await
    }

    /// 获取客户详情 GET /cgi-bin/externalcontact/get
    pub async fn get_external_contact(&self, external_userid: &str) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/externalcontact/get", &[("external_userid", external_userid)]).await
    }

    /// 批量获取客户详情 POST /cgi-bin/externalcontact/batch/get_by_user
    pub async fn batch_get_by_user(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/batch/get_by_user", req).await
    }

    /// 修改客户备注信息 POST /cgi-bin/externalcontact/remark
    pub async fn remark(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/externalcontact/remark", req).await
    }

    // ====== 客户标签管理 ======

    /// 获取企业标签库 GET /cgi-bin/externalcontact/get_corp_tag_list
    pub async fn get_corp_tag_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/get_corp_tag_list", req).await
    }

    /// 添加企业客户标签 POST /cgi-bin/externalcontact/add_corp_tag
    pub async fn add_corp_tag(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/add_corp_tag", req).await
    }

    /// 编辑企业客户标签 POST /cgi-bin/externalcontact/edit_corp_tag
    pub async fn edit_corp_tag(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/externalcontact/edit_corp_tag", req).await
    }

    /// 删除企业客户标签 POST /cgi-bin/externalcontact/del_corp_tag
    pub async fn del_corp_tag(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/externalcontact/del_corp_tag", req).await
    }

    /// 编辑客户企业标签 POST /cgi-bin/externalcontact/mark_tag
    pub async fn mark_tag(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/externalcontact/mark_tag", req).await
    }

    // ====== 联系我与客户入群方式 ======

    /// 配置客户联系「联系我」方式 POST /cgi-bin/externalcontact/add_contact_way
    pub async fn add_contact_way(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/add_contact_way", req).await
    }

    /// 获取企业已配置的「联系我」方式 POST /cgi-bin/externalcontact/get_contact_way
    pub async fn get_contact_way(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/get_contact_way", req).await
    }

    /// 更新企业已配置的「联系我」方式 POST /cgi-bin/externalcontact/update_contact_way
    pub async fn update_contact_way(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/externalcontact/update_contact_way", req).await
    }

    /// 删除企业已配置的「联系我」方式 POST /cgi-bin/externalcontact/del_contact_way
    pub async fn del_contact_way(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/externalcontact/del_contact_way", req).await
    }

    // ====== 客户群管理 ======

    /// 获取客户群列表 POST /cgi-bin/externalcontact/groupchat/list
    pub async fn groupchat_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/groupchat/list", req).await
    }

    /// 获取客户群详情 POST /cgi-bin/externalcontact/groupchat/get
    pub async fn groupchat_get(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/groupchat/get", req).await
    }

    // ====== 消息推送 ======

    /// 创建企业群发 POST /cgi-bin/externalcontact/add_msg_template
    pub async fn add_msg_template(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/add_msg_template", req).await
    }

    /// 提交企业群发任务 POST /cgi-bin/externalcontact/submit_msg_template
    pub async fn submit_msg_template(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/submit_msg_template", req).await
    }

    /// 获取企业的全部群发记录 POST /cgi-bin/externalcontact/get_groupmsg_list_v2
    pub async fn get_groupmsg_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/get_groupmsg_list_v2", req).await
    }

    // ====== 企业服务人员管理 ======

    /// 获取配置了客户联系功能的成员列表 GET /cgi-bin/externalcontact/get_follow_user_list
    pub async fn get_follow_user_list(&self) -> Result<serde_json::Value> {
        self.client.get("/cgi-bin/externalcontact/get_follow_user_list", &[]).await
    }

    // ====== 在职/离职继承 ======

    /// 在职继承 POST /cgi-bin/externalcontact/transfer_customer
    pub async fn transfer_customer(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/transfer_customer", req).await
    }

    /// 离职继承 POST /cgi-bin/externalcontact/resigned/transfer_customer
    pub async fn resigned_transfer_customer(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/resigned/transfer_customer", req).await
    }

    // ====== 统计管理 ======

    /// 获取「联系客户统计」数据 POST /cgi-bin/externalcontact/get_user_behavior_data
    pub async fn get_user_behavior_data(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/externalcontact/get_user_behavior_data", req).await
    }

    /// 通用扩展调用
    pub async fn call_post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    pub async fn call_get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }
}
