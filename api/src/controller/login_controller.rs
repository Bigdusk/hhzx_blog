use axum::extract::State;
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::entity::user;
use crate::entity::user_roles;
use crate::utils::result::ResultUtil;
use crate::utils::jwt;

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(user_info): Json<user::Model>,
) -> Json<Value> {
    //查找用户
    let r = user::Entity::find()
        .filter(
            Condition::all()
                .add(
                    user::Column::Username.eq(user_info.username)
                )
                .add(
                    user::Column::Password.eq(user_info.password)
                )
        )
        .one(&db)
        .await;

    let r = match r {
        Ok(r) => {
            r
        }
        Err(_r) => {
            return ResultUtil::<&str>::error("登录失败");
        }
    };

    match r {
        None => {
            ResultUtil::<&str>::error("登录失败")
        }
        Some(r) => {
            //创建token
            let token = jwt::create_token(r.id);
            match token {
                Ok(r) => { ResultUtil::success(r) }
                Err(r) => { ResultUtil::<&str>::error(r.to_string().as_str()) }
            }
        }
    }
}

pub async fn registration(
    State(db): State<DatabaseConnection>,
    Json(user_info): Json<user::Model>,
) -> Json<Value> {
    if user_info.email.is_none() {
        return ResultUtil::<&str>::error("请输入邮箱");
    }
    let mut user_info = user_info.into_active_model();

    user_info.id = NotSet;
    user_info.nickname = Set(Some("用户".to_string()));
    user_info.create_time = Set(Some(Local::now().naive_local()));
    user_info.update_time = Set(Some(Local::now().naive_local()));

    let r = user_info.insert(&db).await;

    match r {
        Ok(r) => {
            //如果成功将用户关联角色表
            let mut role_association = user_roles::ActiveModel::new();
            role_association.id = NotSet;
            role_association.user_id = Set(r.id);
            //默认用户权限
            role_association.role_id = Set(2);
            let r_role_association = role_association
                .insert(&db)
                .await;
            match r_role_association {
                Ok(_) => {}
                Err(r) => {return ResultUtil::<&str>::error(r.to_string().as_str())}
            }
            //创建token
            let token = jwt::create_token(r.id);
            match token {
                Ok(r) => { ResultUtil::success(r) }
                Err(r) => { ResultUtil::<&str>::error(r.to_string().as_str()) }
            }
        }
        Err(r) => { ResultUtil::<&str>::error(r.to_string().as_str()) }
    }
}
