pub mod department;
pub mod export_import;
pub mod member;
pub mod tag;

use crate::client::WxWorkClient;

pub use department::DepartmentApi;
pub use export_import::{ContactExportApi, ContactImportApi};
pub use member::MemberApi;
pub use tag::TagApi;

/// 通讯录管理模块入口
pub struct ContactsApi<'a> {
    client: &'a WxWorkClient,
}

impl<'a> ContactsApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    pub fn member(&self) -> MemberApi<'_> {
        MemberApi::new(self.client)
    }

    pub fn department(&self) -> DepartmentApi<'_> {
        DepartmentApi::new(self.client)
    }

    pub fn tag(&self) -> TagApi<'_> {
        TagApi::new(self.client)
    }

    pub fn export(&self) -> ContactExportApi<'_> {
        ContactExportApi::new(self.client)
    }

    pub fn import(&self) -> ContactImportApi<'_> {
        ContactImportApi::new(self.client)
    }
}
