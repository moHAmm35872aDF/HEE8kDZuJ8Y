// streaming_processor.rs
// 示例Rust程序，使用WARP框架创建一个简单的大数据流式处理器
// 功能：接受输入流，并处理数据流，实现基本的数据处理功能。

use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Rejection;
use std::collections::HashMap;
use std::error::Error;
use warp::reply::Reply;
use std::fmt::Debug;
use serde_json::json;
use serde_json::Value;

// 定义全局状态，用于存储处理过的数据流统计信息
#[derive(Debug, Clone, Default)]
struct StreamProcessorState {
    processed_count: Arc<Mutex<HashMap<String, usize>>>,
}

// 定义错误类型
#[derive(Debug)]
enum StreamProcessorError {
    InvalidInput,
    InternalError(String),
}

// 实现错误类型为Error trait
impl Error for StreamProcessorError {}

// 将错误类型转换为warp::Rejection
impl warp::reject::Reject for StreamProcessorError {}

// 处理单个数据项的函数
async fn process_single_item(item: Value, state: StreamProcessorState) -> Result<Value, StreamProcessorError> {
    let key = item["key"].to_string();
    let count = state.processed_count.lock().await.entry(key.clone()).or_insert(0);
    *count += 1;
    Ok(json!({ "key": key, "count": count } ))
}

// 创建处理数据流的端点
fn create_stream_processor() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("stream")
        .and(warp::post())
        .and(warp::any().map(move || StreamProcessorState {
            processed_count: Arc::new(Mutex::new(HashMap::new())),
        }))
        .and(warp::body::json::<Value>())
        .and_then(|_: StreamProcessorState, items: Value| {
            async move {
                let results = match items.as_array() {
                    Some(items) => items.iter().map(|item| process_single_item(item.clone(), items.clone())).collect::<Vec<_>>().await,
                    None => vec![Err(StreamProcessorError::InvalidInput)],
                };
                match results.into_iter().collect::<Result<Vec<_>, _>>() {
                    Ok(results) => Ok(warp::reply::json(&results)),
                    Err(_) => Ok(warp::reply::json(&json!({ "error": "Invalid input