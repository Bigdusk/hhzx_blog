use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use sea_orm::DatabaseConnection;
use serde_json::Value;

pub async fn index(
    _db: State<DatabaseConnection>,
    _header_map: HeaderMap
) -> Result<Json<Value>, String>{
    Err("错误".to_string())
}