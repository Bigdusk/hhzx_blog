use axum::extract::{Path, State};
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::entity::user;
use crate::utils::result::ResultUtil;

///添加
pub async fn insert(
    State(db): State<DatabaseConnection>,
    Json(user_info): Json<user::Model>
) -> Json<Value>
{
    let mut category_info = user_info.into_active_model();

    category_info.id = NotSet;
    category_info.create_time = Set(Some(Local::now().naive_local()));
    category_info.update_time = Set(Some(Local::now().naive_local()));

    let r = category_info.insert(&db).await;

    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}
///删除
pub async fn delete(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>
) -> Json<Value>
{
    let r = user::Entity::delete_by_id(user_id)
        .exec(&db)
        .await;

    match r {
        Ok(r) => {
            if r.rows_affected > 0 {
                ResultUtil::success(true)
            }else {
                ResultUtil::<&str>::error("删除失败")
            }
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}
///修改
pub async fn update(
    State(db): State<DatabaseConnection>,
    Json(user_info): Json<user::Model>
) -> Json<Value>
{
    let mut category_info = user_info.into_active_model();
    category_info.username = Set(category_info.username.unwrap());
    category_info.password = Set(category_info.password.unwrap());
    category_info.email = Set(category_info.email.unwrap());
    category_info.nickname = Set(category_info.nickname.unwrap());
    category_info.avatar = Set(category_info.avatar.unwrap());
    category_info.update_time = Set(category_info.update_time.unwrap());
    let r = category_info.update(&db).await;
    match r {
        Ok(_) => {
            ResultUtil::success(true)
        }
        Err(_) => {
            ResultUtil::<&str>::error("修改失败")
        }
    }
}
///查询
pub async fn select_all(
    State(db): State<DatabaseConnection>
) -> Json<Value>
{
    let r = user::Entity::find().all(&db).await;
    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}