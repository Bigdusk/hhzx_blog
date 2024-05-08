use axum::extract::State;
use axum::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet, Order, QueryOrder};
use serde_json::Value;
use crate::entity::timeline;
use crate::utils::result::ResultUtil;

pub async fn timeline_insert(
    State(db): State<DatabaseConnection>,
    Json(timeline_info): Json<timeline::Model>
) -> Json<Value>
{
    let mut timeline_info = timeline_info.into_active_model();

    timeline_info.id = NotSet;

    let r = timeline_info.insert(&db).await;

    match r {
        Ok(_) => {
            ResultUtil::success(true)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}

pub async fn timeline_select_all(
    State(db): State<DatabaseConnection>
) -> Json<Value>
{
    let r = timeline::Entity::find().order_by(timeline::Column::Id, Order::Desc).all(&db).await;
    match r {
        Ok(r) => {ResultUtil::success(r)}
        Err(r) => {ResultUtil::<&str>::error(r.to_string().as_str())}
    }
}