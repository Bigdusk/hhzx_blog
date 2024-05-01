use axum::response::Json;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct ResultUtil<'r, T> {
    pub code: u16,
    pub message: &'r str,
    pub data: Option<T>
}

impl<'r, T> ResultUtil<'r, T>
where
    T: Serialize
{
    pub fn success(date: T) -> Json<Value>{
        Json(json!(Self {
            code: 200,
            message: "请求成功",
            data: Some(date)
        }))

    }

    pub fn error(message: &'r str) -> Json<Value>{
        Json(json!(Self {
            code: 500,
            message,
            data: None
        }))

    }
    pub fn code_and_error(code: u16, message: &'r str) -> Json<Value>{
        Json(json!(Self {
            code,
            message,
            data: None
        }))

    }
}