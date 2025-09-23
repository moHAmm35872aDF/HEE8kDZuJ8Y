use warp::Filter;
use std::error::Error;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::BufReader;
use warp::http::StatusCode;
use warp::reject:: Reject;
# FIXME: 处理边界情况
use std::path::Path;
use std::ffi::OsStr;
use tokio::fs::read_dir;
use tokio::io::AsyncReadExt;
# FIXME: 处理边界情况
use futures::stream;
use crate::csv_processor;

// 定义一个错误类型，用于处理CSV文件处理中的错误
#[derive(Debug)]
pub struct CsvProcessingError(String);

impl CsvProcessingError {
    pub fn new(msg: &str) -> Self {
        CsvProcessingError(msg.to_string())
    }
}

impl warp::reject::Reject for CsvProcessingError {}

impl std::fmt::Display for CsvProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
# NOTE: 重要实现细节
}

impl Error for CsvProcessingError {}

// 定义一个函数来处理单个CSV文件
async fn process_csv_file(file_path: String) -> Result<String, CsvProcessingError> {
# 添加错误处理
    let file = File::open(file_path).map_err(|_| CsvProcessingError::new("Failed to open file"))?;
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|_| CsvProcessingError::new("Failed to read record"))?;
        records.push(record);
    }
    Ok(format!("Processed {} records", records.len()))
}

// 定义一个函数来批量处理目录中的CSV文件
async fn process_csv_files(dir_path: String) -> Result<Vec<String>, CsvProcessingError> {
    let dir = Path::new(&dir_path);
    let mut results = Vec::new();
    let entries = read_dir(dir).await.map_err(|_| CsvProcessingError::new("Failed to read directory"))?;
    for entry in entries {
        let entry = entry.map_err(|_| CsvProcessingError::new("Failed to read entry"))?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("csv")) {
            let file_path = path.to_str().unwrap().to_string();
            let result = process_csv_file(file_path).await;
            results.push(match result {
                Ok(msg) => msg,
# FIXME: 处理边界情况
                Err(_) => "Error processing file".to_string(),
            });
        }
    }
    Ok(results)
# 优化算法效率
}

// 创建一个路由来处理CSV文件批量处理请求
fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("process"))
        .and(warp::path::param())
# 改进用户体验
        .and_then(handle_process_request)
}

// 定义一个处理函数来处理批量处理请求
# 改进用户体验
async fn handle_process_request(dir_path: String) -> Result<impl warp::Reply, warp::Rejection> {
# TODO: 优化性能
    match process_csv_files(dir_path).await {
        Ok(results) => Ok(warp::reply::json(&results)),
        Err(_) => Err(warp::reject::custom(CsvProcessingError::new("Failed to process files"))),
    }
}
# FIXME: 处理边界情况

#[tokio::main]
async fn main() {
    let routes = create_routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
