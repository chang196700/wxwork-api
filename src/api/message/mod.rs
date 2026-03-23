pub mod group;
pub mod send;

use crate::client::WxWorkClient;

pub use group::GroupMessageApi;
pub use send::{MessageSendApi, SendMessageRequest};

/// 消息接收与发送模块入口
pub struct MessageApi<'a> {
    client: &'a WxWorkClient,
}

impl<'a> MessageApi<'a> {
    pub fn new(client: &'a WxWorkClient) -> Self {
        Self { client }
    }

    pub fn send(&self) -> MessageSendApi<'_> {
        MessageSendApi::new(self.client)
    }

    pub fn group(&self) -> GroupMessageApi<'_> {
        GroupMessageApi::new(self.client)
    }
}
