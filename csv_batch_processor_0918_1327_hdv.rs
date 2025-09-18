use warp::Filter;
use std::io::BufReader;
use std::fs::File;
use csv::Reader;
use warp::http::StatusCode;
use warp::http::Response;
use warp::Rejection;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::error::Error;
use std::str::FromStr;
use futures::stream;
use futures::StreamExt;
use tokio::fs::read_dir;
use tokio::fs::read;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;
use tokio::fs::DirEntry;
use tokio_util::codec::{FramedRead, LinesCodec};
use warp::reply::json;

// 定义一个请求结构体，用于解析POST请求的JSON数据
#[derive(Deserialize, Serialize, Debug)]
struct BatchRequest {
    directory: String,
    file_pattern: String,
}

// 定义一个响应结构体，用于返回处理结果
#[derive(Serialize, Debug)]
struct BatchResponse {
    success: bool,
    message: String,
}

// 定义错误类型
#[derive(Debug, Clone)]
enum CsvProcessorError {
    IoError(std::io::Error),
    CsvError(csv::Error),
    ParseError(String),
    Other(String),
}

impl From<std::io::Error> for CsvProcessorError {
    fn from(err: std::io::Error) -> Self {
        CsvProcessorError::IoError(err)
    }
}

impl From<csv::Error> for CsvProcessorError {
    fn from(err: csv::Error) -> Self {
        CsvProcessorError::CsvError(err)
    }
}

impl warp::reject::Reject for CsvProcessorError {}

// 处理CSV文件的逻辑函数
async fn process_csv_file(file_path: PathBuf) -> Result<(), CsvProcessorError> {
    let file = File::open(file_path).map_err(CsvProcessorError::from)?;
    let mut rdr = Reader::from_reader(BufReader::new(file));
    for (i, result) in rdr.records().enumerate() {
            let record = result.map_err(CsvProcessorError::from)?;
            if i == 0 {
                println!("Header: {:?}