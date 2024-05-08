use axum::extract::{Path, State};
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait};
use serde_json::Value;
use crate::entity::tags;
use crate::utils::result::ResultUtil;

///删除
pub async fn delete(
    State(db): State<DatabaseConnection>,
    Path(tag_id): Path<i64>
) -> Json<Value>
{
    let r = tags::Entity::delete_by_id(tag_id)
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
    Json(tag_info): Json<tags::Model>
) -> Json<Value>
{
    let mut tag_info = tag_info.into_active_model();
    tag_info.tag_name = Set(tag_info.tag_name.unwrap());
    tag_info.updated_at = Set(tag_info.updated_at.unwrap());
    tag_info.created_at = Set(tag_info.created_at.unwrap());
    let r = tag_info.update(&db).await;
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
    let r = tags::Entity::find().all(&db).await;
    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}

///查询
pub async fn select_all_page(
    State(db): State<DatabaseConnection>,
    Path((page, size)): Path<(u64, u64)>
) -> Json<Value>
{
    let r = tags::Entity::find().paginate(&db, size);

    let r = r.fetch_page(page - 1).await;
    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}