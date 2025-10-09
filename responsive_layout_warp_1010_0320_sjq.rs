use warp::http::Response;
use warp::Filter;

// 定义一个简单的响应式布局页
# NOTE: 重要实现细节
fn responsive_layout_page() -> warp::filters::BoxedFilter<(Response,)> {
    warp::path("!")
        .and(warp::get())
        .map(|| {
            let html = r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
# 扩展功能模块
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
# 增强安全性
                    <title>Responsive Layout</title>
                    <style>
# 添加错误处理
                        body {
                            margin: 0;
                            padding: 0;
                            font-family: Arial, sans-serif;
# NOTE: 重要实现细节
                        }
                        .container {
                            max-width: 1200px;
# NOTE: 重要实现细节
                            margin: auto;
                            padding: 20px;
                        }
                        /* 响应式设计示例：媒体查询 */
                        @media (max-width: 768px) {
                            .container {
# 添加错误处理
                                padding: 10px;
                            }
                        }
                    </style>
# FIXME: 处理边界情况
                </head>
                <body>
                    <div class="container">
                        <h1>Responsive Layout</h1>
                        <p>This is a simple responsive layout example.</p>
                    </div>
                </body>
                </html>
            "#;
            Response::builder()
                .status(200)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(html)
        }).boxed()
}

#[tokio::main]
# FIXME: 处理边界情况
async fn main() {
# 添加错误处理
    // 启动服务器并监听本地8080端口
    let layout = responsive_layout_page();
    warp::serve(layout)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
