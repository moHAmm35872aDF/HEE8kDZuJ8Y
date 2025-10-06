// 自动批改工具
// 使用RUST和WARP框架实现

use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

// 定义请求和响应的数据结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Submission {
    // 学生提交的代码
    code: String,
    // 提交的测试用例
    test_cases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GradeResult {
    // 测试结果，包括分数和反馈
    score: u8,
    feedback: String,
}

// 模拟的批改逻辑
async fn grade_submission(submission: Submission) -> GradeResult {
    // 这里只是一个简单的示例，实际批改逻辑会更复杂
    let mut score = 0;
    submission.test_cases.iter().for_each(|test_case| {
        // 假设测试用例通过为10分
        if submission.code.contains(test_case) {
            score += 10;
        }
    });

    GradeResult {
        score,
        feedback: format!("Grade: {}", score),
    }
}

// 设置路由和处理逻辑
fn create_grade_service() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("grade"))
        .and(warp::body::json())
        .and_then(|submission: Submission| async {
            let grade_result = grade_submission(submission).await;
            warp::reply::json(&grade_result)
        })
}

#[tokio::main]
async fn main() {
    let grade_service = create_grade_service();
    warp::serve(grade_service).run(([127, 0, 0, 1], 3030)).await;
}
