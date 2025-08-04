use warp::Filter;

// 引入 warp 库，它是构建 RESTful API 的 Rust 异步框架
use warp::http::StatusCode;
use warp::Reject;
use warp::reply;
use thiserror::Error;

// 定义错误类型
#[derive(Debug, Error)]
pub enum MyError {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Not found")]
    NotFound,

    // 可以在这里添加更多的错误类型
}

// 一个简单的响应结构体
struct Response<T> {
    data: T,
}

// 实现 warp 的 Reply trait，以便可以作为响应发送
impl<T> reply::Reply for Response<T>
where
    T: reply::Reply,
{
    fn into_response(self) -> reply::Response {
        self.data.into_response()
    }
}

// 创建一个基本的 GET 请求处理函数
async fn get_example() -> Result<impl reply::Reply, MyError> {
    Ok(Response::<String>("Hello, this is a GET example!".to_string()))
}

// 创建一个基本的 POST 请求处理函数，接受 JSON 数据并返回确认消息
async fn post_example(data: String) -> Result<impl reply::Reply, MyError> {
    // 这里可以添加对数据的验证和处理逻辑
    Ok(Response::<String>(format!("Received POST data: {}", data)))
}

// 设置路由和启动服务器
#[tokio::main]
async fn main() {
    // 定义 GET 路由
    let get_route = warp::path("example")
        .and(warp::get())
        .and_then(get_example);

    // 定义 POST 路由
    let post_route = warp::path("example")
        .and(warp::post())
        .and(warp::body::json()) // 这个 filter 解析 JSON 体
        .and_then(post_example);

    // 组合路由并启动服务器
    let routes = get_route.or(post_route)
        .recover(handle_rejection); // 使用自定义的 rejection handler

    println!("Server running on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// 自定义 rejection handler
async fn handle_rejection(err: Rejection) -> Result<impl reply::Reply, std::convert::Infallible> {
    match err.find::<MyError>() {
        Some(my_error) => Err(my_error),
        None => {
            // 对于未知错误，返回 500 Internal Server Error
            Ok(reply::with_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR))
        },
    }
}
