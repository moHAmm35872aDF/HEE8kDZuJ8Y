// responsive_layout_warp.rs
// 使用 Rust 和 Warp 框架创建响应式布局程序
//
// 功能描述：
// 1. 根据请求头中的 'Accept' 字段返回不同的响应类型。
// 2. 如果 'Accept' 为 'text/html'，则返回 HTML 格式的响应。
# 添加错误处理
// 3. 如果 'Accept' 为 'application/json'，则返回 JSON 格式的响应。
// 4. 如果 'Accept' 为 'text/plain'，则返回纯文本格式的响应。
//
// 错误处理：
// 1. 如果请求头中的 'Accept' 字段未指定或不被支持，则返回 406 Not Acceptable 错误。
//
// 遵循 Rust 最佳实践，代码结构清晰，易于理解，并包含适当的注释和文档。

use warp::http::Response;
use warp::{Rewrite, Filter, reply, http::StatusCode};
use warp::reply::{Reply, json};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

// 定义响应数据结构
#[derive(Serialize)]
struct ResponseData {
    message: String,
}

// 创建一个返回 HTML 响应的 Filter
# 增强安全性
fn html_response() -> impl Filter<Extract = impl Reply, Error = std::convert::Infallible> + Clone {
    warp::path!("html")
        .and(warp::get())
        .map(|| "<html><body><h1>Welcome to Responsive Layout</h1></body></html>")
        .and_then(|html: &str| async move {
# 扩展功能模块
            Ok::<_, std::convert::Infallible>(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
# 扩展功能模块
                .body(html.into())?)
        }).boxed()
}

// 创建一个返回 JSON 响应的 Filter
fn json_response() -> impl Filter<Extract = impl Reply, Error = std::convert::Infallible> + Clone {
    warp::path!("json")
# 增强安全性
        .and(warp::get())
        .map(|| ResponseData { message: "Welcome to Responsive Layout".to_string() })
# FIXME: 处理边界情况
        .and_then(|data: ResponseData| async move {
# 优化算法效率
            Ok::<_, std::convert::Infallible>(Response::builder()
# 优化算法效率
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(json!(data).to_string().into())?)
        }).boxed()
}

// 创建一个返回纯文本响应的 Filter
fn plain_text_response() -> impl Filter<Extract = impl Reply, Error = std::convert::Infallible> + Clone {
    warp::path!("text")
        .and(warp::get())
        .map(|| "Welcome to Responsive Layout")
        .and_then(|text: &str| async move {
            Ok::<_, std::convert::Infallible>(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(text.into())?)
        }).boxed()
}

// 创建一个统一的 Filter 来处理不同的 Accept 类型
fn accept_response() -> impl Filter<Extract = impl Reply, Error = std::convert::Infallible> + Clone {
    warp::any()
        .and(warp::header::<String>("Accept"))
        .and_then(|accept: String| async move {
            match accept.as_str() {
                "text/html" => warp::redirect(warp::path::end() / "html"),
                "application/json" => warp::redirect(warp::path::end() / "json"),
                "text/plain" => warp::redirect(warp::path::end() / "text"),
                _ => Ok(Response::builder()
# FIXME: 处理边界情况
                    .status(StatusCode::NOT_ACCEPTABLE)
                    .body("Not Acceptable".into())?),
            }
        }).boxed()
# 优化算法效率
}

// 启动 Warp 服务器
fn main() {
    let routes = accept_response()
        .or(html_response())
        .or(json_response())
# 扩展功能模块
        .or(plain_text_response());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
# 扩展功能模块
