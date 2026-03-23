use serde::Deserialize;
use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 电子发票 API
pub struct InvoiceApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> InvoiceApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    /// 查询电子发票 POST /cgi-bin/card/invoice/reimburse/getinvoiceinfo
    pub async fn get_invoice_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/card/invoice/reimburse/getinvoiceinfo", req).await
    }

    /// 批量查询电子发票 POST /cgi-bin/card/invoice/reimburse/getinvoiceinfobatch
    pub async fn get_invoice_info_batch(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/card/invoice/reimburse/getinvoiceinfobatch", req).await
    }

    /// 更新发票状态 POST /cgi-bin/card/invoice/reimburse/updateinvoicestatus
    pub async fn update_invoice_status(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/card/invoice/reimburse/updateinvoicestatus", req).await
    }

    /// 批量更新发票状态 POST /cgi-bin/card/invoice/reimburse/updateinvoicestatusonly
    pub async fn update_invoice_status_batch(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/card/invoice/reimburse/updateinvoicestatusonly", req).await
    }
}
