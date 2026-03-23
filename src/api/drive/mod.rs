use crate::client::WxWorkClient;
use crate::error::Result;
use crate::types::common::BaseResponse;

/// 办公 - 微盘 API
pub struct DriveApi<'a> {
    pub(crate) client: &'a WxWorkClient,
}

impl<'a> DriveApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    // ====== 管理空间 ======

    /// 创建空间 POST /cgi-bin/wedrive/space_create
    pub async fn space_create(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedrive/space_create", req).await
    }

    /// 删除空间 POST /cgi-bin/wedrive/space_delete
    pub async fn space_delete(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedrive/space_delete", req).await
    }

    /// 获取空间信息 POST /cgi-bin/wedrive/space_info
    pub async fn space_info(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedrive/space_info", req).await
    }

    /// 获取空间成员列表 POST /cgi-bin/wedrive/spacemember_list
    pub async fn space_member_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedrive/spacemember_list", req).await
    }

    // ====== 管理文件 ======

    /// 获取文件列表 POST /cgi-bin/wedrive/file_list
    pub async fn file_list(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedrive/file_list", req).await
    }

    /// 上传文件 POST /cgi-bin/wedrive/file_upload_part
    pub async fn file_upload(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedrive/file_upload_part", req).await
    }

    /// 下载文件（获取下载链接）POST /cgi-bin/wedrive/file_download
    pub async fn file_download(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedrive/file_download", req).await
    }

    /// 删除文件 POST /cgi-bin/wedrive/file_delete
    pub async fn file_delete(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedrive/file_delete", req).await
    }

    /// 移动文件 POST /cgi-bin/wedrive/file_move
    pub async fn file_move(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedrive/file_move", req).await
    }

    /// 重命名文件 POST /cgi-bin/wedrive/file_rename
    pub async fn file_rename(&self, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/cgi-bin/wedrive/file_rename", req).await
    }

    // ====== 管理文件权限 ======

    /// 新增文件权限 POST /cgi-bin/wedrive/file_acl_add
    pub async fn file_acl_add(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedrive/file_acl_add", req).await
    }

    /// 删除文件权限 POST /cgi-bin/wedrive/file_acl_del
    pub async fn file_acl_del(&self, req: &serde_json::Value) -> Result<BaseResponse> {
        self.client.post("/cgi-bin/wedrive/file_acl_del", req).await
    }

    /// 通用扩展调用
    pub async fn call_post(&self, path: &str, req: &serde_json::Value) -> Result<serde_json::Value> {
        self.client.post(path, req).await
    }
}
