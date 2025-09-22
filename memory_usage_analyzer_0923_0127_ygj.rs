// memory_usage_analyzer.rs
// 一个使用RUST和WARP框架的程序，用于分析内存使用情况。

use warp::Filter;
use std::sync::Arc;
use tokio::runtime::Runtime;
use sys_info;

/// 异步获取内存使用情况的函数
async fn get_memory_usage() -> Result<impl warp::Reply, warp::Rejection> {
    let memory_info = sys_info::mem_info().unwrap_or_else(|_| {
        eprintln!("Failed to get memory info.");
        sys_info::MemInfo::default()
    });

    Ok(warp::reply::json(&memory_info))
}

/// 设置路由并启动服务
fn main() {
    let memory_usage_route = warp::path!("memory")
        .and(warp::get())
        .and_then(get_memory_usage);

    println!("Starting server at http://127.0.0.1:3030
");

    let rt = Arc::new(Runtime::new().unwrap());
    rt.spawn(warp::serve(memory_usage_route).run(([127, 0, 0, 1], 3030)));
}

/// 内存信息结构体
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MemInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub free: u64,
    pub used_percentage: f32,
    pub available_percentage: f32,
}

impl Default for MemInfo {
    fn default() -> Self {
        MemInfo {
            total: 0,
            available: 0,
            used: 0,
            free: 0,
            used_percentage: 0.0,
            available_percentage: 0.0,
        }
    }
}
