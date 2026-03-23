use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 连接微信 - 小程序接入对外收款 API
pub struct MiniprogramPayApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> MiniprogramPayApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 普通支付下单 POST /cgi-bin/miniprogram/pay/papay/createorder
    pub async fn create_order(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/miniprogram/pay/papay/createorder", req).await
    }

    /// 查询订单 POST /cgi-bin/miniprogram/pay/papay/getorder
    pub async fn get_order(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/miniprogram/pay/papay/getorder", req).await
    }

    /// 退款 POST /cgi-bin/miniprogram/pay/papay/refund
    pub async fn refund(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/miniprogram/pay/papay/refund", req).await
    }

    /// 查询退款 POST /cgi-bin/miniprogram/pay/papay/getrefund
    pub async fn get_refund(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/miniprogram/pay/papay/getrefund", req).await
    }

    /// 下载账单 POST /cgi-bin/miniprogram/pay/papay/downloadbill
    pub async fn download_bill(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/miniprogram/pay/papay/downloadbill", req).await
    }
}
