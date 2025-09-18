use warp::Filter;
use html_escape::encode_html;
use warp::Rejection;
use warp::http::StatusCode;
use std::str::Utf8Error;
use warp::reply::Response;
use warp::reply::Reply;
use warp::reject::Reject;

// 定义自定义错误类型XssError，用于处理XSS防护相关错误
#[derive(Debug, Clone)]
struct XssError;

impl Reject for XssError {}

// 创建一个简单的XSS防护过滤器
fn xss_protection_filter() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::any()
        .map(move || {
            std::env::var("QUERY_STRING").map(|qs| {
                let mut is_xss = false;
                // 检查查询字符串中是否包含潜在的XSS攻击向量
                if qs.contains("<script>") || qs.contains("</script>") {
                    is_xss = true;
                }
                // 可以添加更多的XSS检测逻辑
                
                if is_xss {
                    // 如果检测到XSS，返回错误响应
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(XssError)
                        .unwrap()
                } else {
                    Response::builder()
                        .status(StatusCode::OK)
                        .body("No XSS detected")
                        .unwrap()
                }
            }).unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::OK)
                    .body("No query string provided")
                    .unwrap()
            })
        })
        .untuple_one()
        .and_then(handle_rejection)
}

// 处理拒绝情况，返回适当的响应
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found"))
    } else if let Some(XssError) = err.find() {
        Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("XSS detected in request"))
    } else {
        // 其他错误处理
        Err(err)
    }
}

// 启动服务的main函数
#[tokio::main]
async fn main() {
    let xss_protection = xss_protection_filter();
    warp::serve(xss_protection)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 模块文档
/// This module provides a simple XSS protection filter for Warp applications.
/// It checks the query string for potential XSS attack vectors and returns an error response if any are found.
pub mod xss_protection {
    //! 这里可以添加更多XSS防护相关的函数和结构体
}

/// Encodes HTML to prevent XSS attacks.
///
/// This function takes a string slice and returns its HTML-encoded version,
/// which can be safely included in an HTML document without causing XSS vulnerabilities.
///
/// # Examples
///
/// ```rust
/// let html = "<b>Hello, world!</b>";
/// let encoded = encode_html(html);
/// assert_eq!(encoded, "&lt;b&gt;Hello, world!&lt;/b&gt;");
pub fn encode_html(input: &str) -> String {
    encode_html(input)
}
