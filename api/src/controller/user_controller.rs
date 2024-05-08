use std::error::Error;
use axum::extract::{Path, State};
use axum::Json;
use http::HeaderMap;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, TransactionTrait};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::entity::user;
use crate::entity::user_roles;
use crate::utils::result::ResultUtil;
use crate::utils::jwt;
///获取登录个人信息
pub async fn login_user_info(
    State(db): State<DatabaseConnection>,
    header_map: HeaderMap,
)  -> Json<Value>
{
    let r = jwt::get_user_info(&header_map, &db).await;
    match r {
        Ok(mut r) => {
            r.password = "保护".to_string();
            ResultUtil::success(r)
        }
        Err(_) => {ResultUtil::<&str>::code_and_error(10010, "获取用户登录信息失败")}
    }
}
///删除
pub async fn delete(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>
) -> Json<Value>
{
    if user_id == 0 {
        return ResultUtil::<&str>::error("默认权限无法修改")
    }
    let r = user_delete_service(db, user_id).await;

    match r {
        Ok(r) => {r}
        Err(r) => {ResultUtil::<&str>::error(r.to_string().as_str())}
    }

}

async fn user_delete_service(db: DatabaseConnection, user_id: i64)
-> Result<Json<Value>, Box<dyn std::error::Error>>
{
    let txn = db.begin().await?;
    user::Entity::delete_by_id(user_id)
        .exec(&txn)
        .await?;
    //删除对应权限
    user_roles::Entity::delete_many()
        .filter(
            Condition::all()
                .add(
                    user_roles::Column::UserId.eq(user_id)
                )
        )
        .exec(&txn)
        .await?;
    txn.commit().await?;

    Ok(ResultUtil::success(true))
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