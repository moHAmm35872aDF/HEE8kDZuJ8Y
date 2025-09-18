// xss_protection.rs

use warp::Filter;
use std::error::Error;
use html_escape::encode_html;
use warp::http::Response;

// 定义一个简单的结构体来存储我们的XSS保护函数
struct XssProtection;

impl XssProtection {
    // 创建一个新的XssProtection实例
    pub fn new() -> Self {
        XssProtection
    }

    // 定义一个函数来清除输入中的XSS攻击
    pub fn sanitize_input(&self, input: String) -> String {
        // 将输入转换为HTML安全的字符串
        encode_html(&input).to_string()
    }

    // 定义一个函数来检查并清除XSS攻击
    pub async fn check_for_xss(&self, input: String) -> Result<String, Box<dyn Error>> {
        let sanitized = self.sanitize_input(input);
        // 这里可以添加更复杂的逻辑来进一步检查XSS攻击
        Ok(sanitized)
    }
}

// 创建一个简单的路由来处理XSS检查请求
fn with_xss_protection() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("xss")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1024)) // 限制请求体大小为1MB
        .and(warp::body::string())
        .and(with_state(XssProtection::new()))
        .and_then(handle_xss_check)
}

// 状态函数，允许我们将`XssProtection`状态传递给处理函数
fn with_state(state: XssProtection) -> impl Filter<Extract = (XssProtection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

// 处理XSS检查请求的函数
async fn handle_xss_check(xss_protection: XssProtection, input: String) -> Result<impl warp::Reply, warp::Rejection> {
    match xss_protection.check_for_xss(input).await {
        Ok(sanitized) => Ok(warp::reply::json(&{"message": "XSS check passed", "sanitized_input": sanitized})),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// 启动WARP服务器的主函数
#[tokio::main]
async fn main() {
    let xss_filter = with_xss_protection();

    warp::serve(xss_filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

