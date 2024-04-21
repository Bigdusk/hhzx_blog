use axum::{
    response::Response,
    middleware::Next,
    extract::Request,
};
use axum::http::HeaderValue;

///自定义中间件方法
pub async fn cors(
    request: Request,
    next: Next,
) -> Response {

    let mut  response = next.run(request).await;

    response
        .headers_mut()
        .insert("access-control-allow-origin", HeaderValue::from_static("*"));
    response.headers_mut().insert(
        "access-control-allow-methods",
        HeaderValue::from_static("POST, GET, OPTIONS, PATCH, DELETE"),
    );
    response.headers_mut().insert(
        "access-control-allow-headers",
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .insert("access-control-max-age", HeaderValue::from_static("86400"));

    response
}
