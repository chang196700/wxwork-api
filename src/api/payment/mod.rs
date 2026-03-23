use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 连接微信 - 企业支付 API
pub struct PaymentApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> PaymentApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 向员工付款 POST /cgi-bin/pay/transfer_to_user
    pub async fn transfer_to_user(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/pay/transfer_to_user", req).await
    }

    /// 查询付款记录 POST /cgi-bin/pay/queryrecord
    pub async fn query_record(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/pay/queryrecord", req).await
    }

    /// 企业红包 POST /cgi-bin/pay/sendworkwxredpack
    pub async fn send_redpack(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/pay/sendworkwxredpack", req).await
    }

    /// 查询红包记录 POST /cgi-bin/pay/queryworkwxredpack
    pub async fn query_redpack(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/pay/queryworkwxredpack", req).await
    }

    /// 创建对外收款账户 POST /cgi-bin/pay/create_account
    pub async fn create_account(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/pay/create_account", req).await
    }

    /// 对外收款 POST /cgi-bin/pay/get_receiving_money
    pub async fn get_receiving_money(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/pay/get_receiving_money", req).await
    }

    /// 向员工收款 POST /cgi-bin/pay/transfer_from_user
    pub async fn transfer_from_user(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/pay/transfer_from_user", req).await
    }
}
