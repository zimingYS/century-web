use serde::{Deserialize, Serialize};

/// 用户登录请求。
///
/// 该结构体表示客户端发送到 `/login` 接口的 JSON 数据。
/// 使用 `Deserialize` 将 HTTP Body 自动反序列化为 Rust 类型。
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// 用户名（登录唯一标识）
    pub username: String,

    /// 用户密码（明文仅存在于请求阶段，后续会进行哈希处理）
    pub password: String,
}

/// 用户登录响应
///
/// 该结构体表示 `/login` 接口成功返回后的数据
/// 注意：这里只返回允许暴露给客户端的信息，
/// 不应包含密码等敏感字段。
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// 用户唯一标识
    pub id: u64,

    /// 用户名
    pub username: String,
}
