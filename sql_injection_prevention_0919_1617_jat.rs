use warp::Filter;
use tokio::sync::Mutex;
use warp::http::Response;
use warp::reject::Reject;
use tokio_postgres::{NoTls, Error as PgError};
use tokio_postgres::Client;
use lazy_static::lazy_static;
use regex::Regex;

// 定义一个简单的错误类型
#[derive(Debug)]
struct SqlInjectionError;

// 实现Reject特性，用于处理SQL注入错误
impl Reject for SqlInjectionError {}

// 定义数据库客户端
lazy_static! {
    static ref DB_CLIENT: Mutex<Client> = {
        Mutex::new(Client::connect("host=localhost user=postgres", NoTls).unwrap())
    };
}

// 防止SQL注入的函数
async fn prevent_sql_injection(input: String) -> Result<String, SqlInjectionError> {
    // 使用正则表达式来检测潜在的SQL注入
    let sql_injection_regex = Regex::new(r"""--|;|\b(SELECT|INSERT|UPDATE|DELETE|DROP|GRANT|REVOKE|UNION|EXEC|EXECUTE|ALTER|CREATE|ALTER TABLE|TRUNCATE|RENAME|LOAD_FILE)\b""").unwrap();
    if sql_injection_regex.is_match(&input) {
        // 如果检测到SQL注入，返回错误
        Err(SqlInjectionError)
    } else {
        // 如果没有检测到SQL注入，返回原始输入
        Ok(input)
    }
}

// 创建一个简单的API端点，用于演示防止SQL注入的功能
fn prevent_sql_injection_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("prevent_sql_injection")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16)) // 限制请求体大小
        .and(warp::body::bytes())
        .then(|body: warp::hyper::body::Bytes| async move {
            // 将请求体解码为字符串
            let input = match String::from_utf8(body.to_vec()) {
                Ok(s) => s,
                Err(_) => return Response::builder()
                    .status(400)
                    .body("Invalid UTF-8 sequence".into())
                    .unwrap_err(),
            };

            // 调用防止SQL注入的函数
            match prevent_sql_injection(input).await {
                Ok(_) => Response::builder()
                    .status(200)
                    .body("Input is safe".into())
                    .unwrap(),
                Err(_) => Response::builder()
                    .status(400)
                    .body("SQL Injection detected".into())
                    .unwrap_err(),
            }
        })
}

#[tokio::main]
async fn main() {
    // 启动WARP服务器
    let api = prevent_sql_injection_api();
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
