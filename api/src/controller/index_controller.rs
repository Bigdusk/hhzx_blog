use axum::{Json, http::StatusCode};
use axum::extract::State;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::{json, Value};
use crate::entity::user;

pub async fn index(
    db: State<DatabaseConnection>
) -> Result<Json<Value>, StatusCode>{
    let db = &db.0;
    let r = user::Entity::find().all(db).await.unwrap();
    Ok(Json(json!(r)))
}