// restful_api_server.rs
// 这是一个使用RUST和WARP框架构建的RESTful API服务器。

use warp::Filter;

// 定义主函数
#[tokio::main]
async fn main() {
    // 定义路由
    let api = warp::path("hello")
        .and(warp::get())
        .map(|| warp::reply::json(&"Hello, World!"));

    // 启动服务器
    println!("Server running on http://localhost:3030/");
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// 使用/warp::get()来定义一个GET请求的处理器
// 使用/warp::path()定义路径
// 使用/warp::reply::json()来返回JSON响应
