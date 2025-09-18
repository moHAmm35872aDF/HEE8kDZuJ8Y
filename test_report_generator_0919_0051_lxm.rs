use warp::Filter;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use serde_json::json;
use std::path::Path;
use std::error::Error;

// 定义测试报告数据结构
#[derive(Serialize)]
struct TestReport {
    description: String,
    status: String,
    details: String,
}

// 创建一个简单的测试报告生成器
async fn create_test_report(description: String, status: String, details: String) -> Result<TestReport, Box<dyn Error>> {
    let report = TestReport {
        description,
        status,
        details,
    };

    // 将测试报告写入文件
    let file_path = Path::new("./test_report.json");
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    let json_report = serde_json::to_string_pretty(&report)?;
    writer.write_all(json_report.as_bytes())?;

    Ok(report)
}

// 设置WARP过滤器，处理POST请求以生成测试报告
fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("test_report"))
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            let description = body["description"].as_str().unwrap().to_string();
            let status = body["status"].as_str().unwrap().to_string();
            let details = body["details"].as_str().unwrap().to_string();

            warp::reply::json(&warp::anyhow::Result::<TestReport>::ok(create_test_report(description, status, details).await.unwrap_or_else(|e| {
                panic!("Error creating report: {}", e)
            })));
        })
}

#[tokio::main]
async fn main() {
    // 启动WARP服务器
    warp::serve(routes()).run(([127, 0, 0, 1], 3030)).await;
}
