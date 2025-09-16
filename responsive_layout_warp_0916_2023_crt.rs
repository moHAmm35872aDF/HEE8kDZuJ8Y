use warp::Filter;

// 定义响应式布局的路由
fn responsive_layout_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("responsive")
        .and(warp::get())
        .map(|| {
            // 响应HTML内容
            // 在实际项目中，HTML内容应该存储在文件中或数据库中，并动态生成
            let html_content = "
                <html>
                    <head>
                        <title>Responsive Layout</title>
                        <style>
                            body {
                                margin: 0;
                                padding: 0;
                                font-family: Arial, sans-serif;
                            }
                            .container {
                                width: 100%;
                                max-width: 1200px;
                                margin: auto;
# FIXME: 处理边界情况
                            }
# 优化算法效率
                            @media (max-width: 600px) {
                                .container {
                                    padding: 10px;
                                }
                            }
# NOTE: 重要实现细节
                        </style>
                    </head>
                    <body>
                        <div class="container">
# FIXME: 处理边界情况
                            <h1>This is a responsive layout!</h1>
                            <p>Resize the browser window to see the responsive design in action.</p>
                        </div>
                    </body>
                </html>";

            warp::reply::html(html_content)
        }).recover(handle_rejection)
}

// 错误处理函数
fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.find::<warp::reject::Not Found>().is_some() || err.find::<warp::reject::Method Not Allowed>().is_some() {
        Ok(warp::reply::with_status("Not found", warp::http::StatusCode::NOT_FOUND))
    } else {
        Err(err)
    }
# 添加错误处理
}

#[tokio::main]
async fn main() {
    println!("Server started at http://127.0.0.1:3030/");
    warp::serve(responsive_layout_route())
        .run(([127, 0, 0, 1], 3030))
        .await;
}