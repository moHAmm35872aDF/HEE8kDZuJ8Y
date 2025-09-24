use warp::Filter;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;
use serde_json;
use thiserror::Error;

// 定义我们自己错误类型，用于处理CSV文件读取和处理过程中的错误
#[derive(Debug, Error)]
pub enum BatchProcessorError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("CSV Error: {0}")]
    CsvError(#[from] csv::Error),
    #[error("Deserialize Error: {0}")]
    DeserializeError(#[from] serde_json::Error),
}

// 定义CSV行数据结构，这里需要根据实际CSV文件的列结构来调整
#[derive(Debug, Deserialize)]
pub struct CsvRow {
    // 示例：将第一列作为id，第二列作为name
    id: String,
    name: String,
    // ...添加更多字段
}

// 定义处理CSV文件的函数
async fn process_csv_file(file_path: String) -> Result<String, BatchProcessorError> {
    // 打开文件并创建一个读取器
    let file = File::open(file_path).map_err(BatchProcessorError::IoError)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));

    // 读取CSV文件，将每行转换为CsvRow结构
    let mut records: Vec<CsvRow> = Vec::new();
    for result in rdr.deserialize() {
        let record: CsvRow = result.map_err(BatchProcessorError::CsvError)?;
        records.push(record);
    }

    // 处理CSV数据，这里可以根据需要进行扩展
    // 例如：打印出来
    for record in records {
        println!("ID: {}, Name: {}", record.id, record.name);
    }

    // 返回处理结果的JSON字符串
    Ok(serde_json::to_string(&records).map_err(BatchProcessorError::DeserializeError)?)
}

// 创建Warp过滤器来处理POST请求，并将上传的文件路径传递给process_csv_file函数
fn main() {
    // 定义一个POST请求的处理函数，这里为了简化，直接使用端口8080，没有使用路由
    let process_csv = warp::post()
        .and(warp::path("process"))
        .and(warp::body::json()) // 假设客户端以JSON格式发送文件路径
        .map(|file_path: String| warp::reply::json(&process_csv_file(file_path).unwrap_or_else(|e| {
            // 在实际应用中，这里应该返回错误信息，而不是直接打印
            eprintln!("Error processing CSV file: {:?}