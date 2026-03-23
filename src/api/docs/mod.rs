use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 文档 API
pub struct DocsApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> DocsApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    // ====== 管理文档 ======

    /// 新建文档 POST /cgi-bin/wedoc/create_doc
    pub async fn create_doc(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedoc/create_doc", req).await
    }

    /// 重命名文档 POST /cgi-bin/wedoc/rename_doc
    pub async fn rename_doc(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedoc/rename_doc", req).await
    }

    /// 删除文档 POST /cgi-bin/wedoc/del_doc
    pub async fn del_doc(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedoc/del_doc", req).await
    }

    /// 获取文档基础信息 POST /cgi-bin/wedoc/get_doc_base_info
    pub async fn get_doc_base_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedoc/get_doc_base_info", req).await
    }

    /// 获取文档详情 POST /cgi-bin/wedoc/document/get_doc_detail
    pub async fn get_doc_detail(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedoc/document/get_doc_detail", req).await
    }

    // ====== 设置文档权限 ======

    /// 设置成员文档权限 POST /cgi-bin/wedoc/auth/set_doc_member
    pub async fn set_doc_member(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedoc/auth/set_doc_member", req).await
    }

    /// 获取文档成员信息 POST /cgi-bin/wedoc/auth/get_doc_member
    pub async fn get_doc_member(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedoc/auth/get_doc_member", req).await
    }

    /// 删除文档成员 POST /cgi-bin/wedoc/auth/del_doc_member
    pub async fn del_doc_member(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedoc/auth/del_doc_member", req).await
    }

    /// 设置文档安全设置 POST /cgi-bin/wedoc/auth/set_doc_security
    pub async fn set_doc_security(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedoc/auth/set_doc_security", req).await
    }

    // ====== 管理表格/文档内容 ======

    /// 获取表格内容 POST /cgi-bin/wedoc/spreadsheet/get_sheet_properties
    pub async fn get_sheet_properties(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedoc/spreadsheet/get_sheet_properties", req).await
    }

    /// 向表格追加行 POST /cgi-bin/wedoc/spreadsheet/append_rows
    pub async fn append_rows(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedoc/spreadsheet/append_rows", req).await
    }

    /// 通用扩展调用
    pub async fn call_post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }

    pub async fn call_get(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.client.get(path, query).await
    }
}
