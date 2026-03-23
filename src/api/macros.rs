/// 生成一个调用企业微信 GET API 的异步方法
///
/// 用法：
/// ```ignore
/// api_get!(get_user, "/cgi-bin/user/get", GetUserRequest, GetUserResponse);
/// ```
/// 展开为：
/// ```ignore
/// pub async fn get_user(&self, req: GetUserRequest) -> crate::error::Result<GetUserResponse> {
///     let pairs = req.to_query_pairs();
///     let refs: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
///     self.client.get::<GetUserResponse>("/cgi-bin/user/get", &refs).await
/// }
/// ```
#[macro_export]
macro_rules! api_get {
    ($method:ident, $path:expr, $req_ty:ty, $resp_ty:ty) => {
        pub async fn $method(
            &self,
            req: $req_ty,
        ) -> $crate::error::Result<$resp_ty> {
            let pairs = $crate::api::macros::to_query_pairs(&req);
            let refs: Vec<(&str, &str)> =
                pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            self.client.get::<$resp_ty>($path, &refs).await
        }
    };
}

/// 生成一个调用企业微信 POST JSON API 的异步方法
///
/// 用法：
/// ```ignore
/// api_post!(create_user, "/cgi-bin/user/create", CreateUserRequest, BaseResponse);
/// ```
#[macro_export]
macro_rules! api_post {
    ($method:ident, $path:expr, $req_ty:ty, $resp_ty:ty) => {
        pub async fn $method(
            &self,
            req: $req_ty,
        ) -> $crate::error::Result<$resp_ty> {
            self.client.post::<$req_ty, $resp_ty>($path, &req).await
        }
    };
}

/// 生成一个返回 `BaseResponse` 并检查 errcode 的 POST 方法
///
/// 成功时返回 `()`，失败时返回 `WxWorkError::ApiError`
#[macro_export]
macro_rules! api_post_ok {
    ($method:ident, $path:expr, $req_ty:ty) => {
        pub async fn $method(&self, req: $req_ty) -> $crate::error::Result<()> {
            let resp = self
                .client
                .post::<$req_ty, $crate::types::common::BaseResponse>($path, &req)
                .await?;
            $crate::client::WxWorkClient::check_base(resp)
        }
    };
}

/// 将实现了 `serde::Serialize` 的结构体序列化为 query pair 列表
///
/// 仅支持扁平化的字符串/数字字段（不含嵌套对象），用于 GET 请求拼 query。
pub fn to_query_pairs<T: serde::Serialize>(req: &T) -> Vec<(String, String)> {
    let value = serde_json::to_value(req).unwrap_or(serde_json::Value::Null);
    let mut pairs = Vec::new();
    if let serde_json::Value::Object(map) = value {
        for (k, v) in map {
            let s = match &v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => continue,
                // 数组/对象字段序列化为 JSON 字符串（适配部分企业微信 GET 参数）
                other => other.to_string(),
            };
            pairs.push((k, s));
        }
    }
    pairs
}
