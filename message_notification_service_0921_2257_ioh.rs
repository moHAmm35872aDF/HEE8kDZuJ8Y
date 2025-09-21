use warp::Filter;

// 定义消息通知的结构体
#[derive(Debug, Clone)]
struct Notification {
    message: String,
}

// Async handler函数用于处理通知请求
async fn notify(notification: Notification) -> Result<impl warp::Reply, warp::Rejection> {
    // 这里可以添加发送通知的逻辑，例如打印消息到控制台
    println!("Received notification: {}", notification.message);
    
    // 如果发送成功，返回OK状态码
    Ok(warp::http::StatusCode::OK)
}

#[tokio::main]
async fn main() {
    // 设置通知路径
    let notification_route = warp::path("notify")
        .and(warp::body::json()) // 接收JSON请求体
        .and_then(notify); // 处理通知请求

    // 启动服务
    println!("Starting message notification service on http://localhost:3030");
    warp::serve(notification_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 用于过滤和解析JSON请求体
fn with_notification() -> impl Filter<Extract = (Notification,), Error = warp::Rejection> + Clone {
    warp::body::json().map(|json: serde_json::Value| {
        // 解析JSON请求体为Notification结构体
        if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
            Notification { message: message.to_string() }
        } else {
            // 如果消息字段不存在，返回一个400 Bad Request错误
            warp::reject::custom(MissingField)
        }
    })
}

// 自定义错误类型
#[derive(Debug)]
struct MissingField;

impl warp::reject::Reject for MissingField {}

impl std::fmt::Display for MissingField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing 'message' field in request body")
    }
}

impl std::error::Error for MissingField {}
