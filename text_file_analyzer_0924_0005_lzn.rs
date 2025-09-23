// text_file_analyzer.rs
use warp::Filter;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use serde_json::json;
use serde_json::Value;

// 定义一个结构体来存储分析结果
#[derive(Debug, Clone)]
struct AnalysisResult {
    file_name: String,
    line_count: usize,
    word_count: usize,
    character_count: usize,
}

// 异步函数用于分析文本文件
async fn analyze_text_file(path: String) -> Result<impl warp::Reply, warp::Rejection> {
    // 确保路径有效且文件存在
    let path = Path::new(&path);
    if !path.exists() || !path.is_file() {
        return Ok(warp::reply::json(&json!({"error": "File not found or not a file"})));
    }

    // 读取文件内容
    let mut file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => return Ok(warp::reply::json(&json!({"error": format!("Failed to open file: {}", e)}))),
    };

    // 初始化计数器
    let mut line_count = 0;
    let mut word_count = 0;
    let mut character_count = 0;

    // 读取文件的每一行
    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap_or(0) != 0 {
        // 计算行数、字数和字符数
        line_count += 1;
        word_count += line.split_whitespace().count();
        character_count += line.len();
        line.clear();
    }

    // 返回分析结果
    Ok(warp::reply::json(&json!({"file_name": path.display().to_string(),
                                   "line_count": line_count,
                                   "word_count": word_count,
                                   "character_count": character_count}
    )))
}

// 创建Warp过滤器处理POST请求
fn create_analyzer_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("analyze"))
        .and(warp::path::param::<String>())
        .and_then(analyze_text_file)
}

#[tokio::main]
async fn main() {
    // 启动Warp服务
    let filter = create_analyzer_filter();
    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}
