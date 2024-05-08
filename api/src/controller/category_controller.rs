use axum::extract::{Path, State};
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::entity::{category, article};
use crate::utils::result::ResultUtil;

///添加
pub async fn insert(
    State(db): State<DatabaseConnection>,
    Json(category_info): Json<category::Model>
) -> Json<Value>
{
    let mut category_info = category_info.into_active_model();

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
    Path(category_id): Path<i64>
) -> Json<Value>
{
    //判断是否还有文章
    let article_list = article::Entity::find()
        .filter(
            Condition::all()
                .add(
                    article::Column::CategoryId.eq(category_id)
                )
        )
        .all(&db)
        .await;
    match article_list {
        Ok(r) => {
            if r.len() > 0 {
                return ResultUtil::<&str>::error("该分类还有关联文章")
            }
        }
        Err(r) => {return ResultUtil::<&str>::error(r.to_string().as_str())}
    }

    //执行删除
    let r = category::Entity::delete_by_id(category_id)
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
    Json(category_info): Json<category::Model>
) -> Json<Value>
{
    let mut category_info = category_info.into_active_model();
    category_info.category_name = Set(category_info.category_name.unwrap());
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
    let r = category::Entity::find().all(&db).await;
    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}