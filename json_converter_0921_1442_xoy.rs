use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use warp::reject::Reject;

// 定义一个简单的错误类型
#[derive(Debug)]
struct JsonError;

// 实现 Reject trait，以便可以在 warp 中处理这个错误
impl Reject for JsonError {}

// 定义输入数据结构
#[derive(Deserialize)]
struct InputData {
    value: String,
}

// 定义输出数据结构
#[derive(Serialize)]
struct OutputData {
    transformed: Value,
}

// 转换 JSON 数据的函数
async fn convert_json(input: InputData) -> Result<impl warp::Reply, JsonError> {
    let input_json: Result<Value, _> = serde_json::from_str(&input.value);
    match input_json {
        Ok(json_value) => {
            // 这里可以添加任何转换逻辑
            // 例如，只是简单地返回相同的 JSON
            Ok(warp::reply::json(&OutputData {
                transformed: json_value,
            }))
        },
        Err(e) => {
            // 错误处理
            eprintln!("JSON parsing error: {}", e);
            Err(JsonError)
        },
    }
}

// 设置 warp 路由
fn main() {
    let json_route = warp::path!("json")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(convert_json);

    warp::serve(json_route)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
