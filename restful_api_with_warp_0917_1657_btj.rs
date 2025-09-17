use warp::Filter;

// 定义一个简单的用户模型
#[derive(serde::Serialize)]
struct User {
    id: u32,
    name: String,
}

// 创建一个用户列表，用于演示
fn get_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ]
}

// 创建一个API端点，返回用户列表
fn users_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(warp::get())
        .map(|| {
            get_users()
        })
        .and_then(|users: Vec<User>| async move {
            warp::reply::json(&users)
        })
}

// 定义程序的入口点
#[tokio::main]
async fn main() {
    // 设置日志
    warp::log::info!("Server starting");

    // 启动服务
    let users_route = users_route();

    let routes = warp::any()
        .and(users_route)
        .recover(handle_rejection);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 错误处理函数，用于统一处理错误
fn handle_rejection(err: warp::Rejection) -> warp::Rejection {
    if err.is_not_found() {
        warp::reject::not_found()
    } else {
        warp::reject::custom(err)
    }
}

// 函数注释
// 程序启动时调用此函数，设置日志级别
fn setup_logging() {
    std::env::set_var("fmt_log_debug", "1");
    std::env::set_var("fmt_log_info", "1");
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
}

// 函数注释
// 此处添加其他API端点
// fn other_api_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     // 定义和实现其他API端点
// }

// 函数注释
// 配置依赖项
fn main() {
    setup_logging();
    run_server().await;
}

// 函数注释
// 运行WARP服务器
async fn run_server() {
    // 这里可以添加其他路由和配置
    let routes = users_route();
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
