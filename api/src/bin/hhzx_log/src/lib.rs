use axum::{
    response::Response,
    middleware::Next,
    extract::Request,
};

///自定义中间件方法
pub async fn log(
    request: Request,
    next: Next,
) -> Response {
    // 对请求做一些处理
    //println!("{:#?}", request);
    //......

    //调用下一个中间价
    let response = next.run(request).await;

    //......

    // 对响应做一些处理，返回响应
    response
}