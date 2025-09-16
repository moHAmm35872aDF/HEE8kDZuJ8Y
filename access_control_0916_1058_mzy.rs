use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

// 定义用户身份验证信息
struct AuthInfo {
    users: Arc<Mutex<HashMap<String, String>>>,
}

// 实现新方法来创建一个新的AuthInfo实例
impl AuthInfo {
    pub fn new() -> Self {
        let users = HashMap::new();
        let users = Arc::new(Mutex::new(users));

        AuthInfo { users }
    }
}

// 定义一个函数来添加用户
fn add_user(auth_info: Arc<AuthInfo>, username: String, password: String) {
    let mut users = auth_info.users.lock().unwrap();
    users.insert(username, password);
}

// 定义一个函数来实现权限检查的逻辑
async fn check_auth(user: String, pass: String, auth_info: Arc<AuthInfo>) -> Result<&'static str, warp::Rejection> {
    let users = auth_info.users.lock().await;
    if users.get(&user) == Some(&pass) {
        Ok("Access granted")
    } else {
        Err(warp::reject::custom(Unauthorized))
    }
}

// 定义一个结构体来表示未授权错误
struct Unauthorized;

// 实现Reject trait来定义未授权错误的行为
impl warp::reject::Reject for Unauthorized {}

// 创建一个Warp过滤器来处理登录请求
fn login_route(auth_info: Arc<AuthInfo>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("Authorization"))
        .and(with_auth_info(auth_info))
        .and_then(|user: String, pass: String, auth_header: String, auth_info: Arc<AuthInfo>| {
            if auth_header == "Bearer token" {
                check_auth(user, pass, auth_info)
            } else {
                Err(warp::reject::custom(Unauthorized))
            }
        })
        .map(warp::reply)
}

// 创建一个Warp过滤器来处理受保护的路由
fn protected_route(auth_info: Arc<AuthInfo>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("protected")
        .and(warp::get())
        .and(with_auth_info(auth_info))
        .and_then(|auth_info: Arc<AuthInfo>| async move {
            Ok(warp::reply::json(&"Protected data"))
        }).recover(handle_rejection)
}

// 创建一个Warp过滤器来包装AuthInfo
fn with_auth_info(auth_info: Arc<AuthInfo>) -> impl Filter<Extract = Arc<AuthInfo>, Error = warp::Rejection> + Clone {
    warp::any().map(move || auth_info.clone())
}

// 处理未授权的请求
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.find::<Unauthorized>().is_some() {
        Ok(warp::reply::with_status("Unauthorized", warp::http::StatusCode::UNAUTHORIZED))
    } else {
        Err(err)
    }
}

#[tokio::main]
async fn main() {
    let auth_info = AuthInfo::new();
    add_user(Arc::new(auth_info.clone()), "user1".to_string(), "pass1".to_string());

    let login_route = login_route(Arc::new(auth_info.clone()));
    let protected_route = protected_route(Arc::new(auth_info.clone()));

    let routes = login_route.or(protected_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}