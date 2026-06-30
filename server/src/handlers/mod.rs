//! HTTP 请求处理层。
//!
//! Handler 负责接收 HTTP 请求、
//! 调用 Service 完成业务，
//! 最终构造 HTTP 响应。
//!
//! 本层不直接访问数据库，也不编写复杂业务逻辑。

pub mod user;
