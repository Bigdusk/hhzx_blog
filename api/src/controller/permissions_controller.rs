use axum::extract::{Path, State};
use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::entity::{permissions, roles, roles_permissions, user_roles};
use crate::utils::result::ResultUtil;

///添加
pub async fn permissions_insert(
    State(db): State<DatabaseConnection>,
    Json(permissions_info): Json<permissions::Model>
) -> Json<Value>
{
    let mut permissions_info = permissions_info.into_active_model();

    permissions_info.id = NotSet;

    let r = permissions_info.insert(&db).await;

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
pub async fn permissions_delete(
    State(db): State<DatabaseConnection>,
    Path(permissions_id): Path<i64>
) -> Json<Value>
{
    if permissions_id == 0 {
        return ResultUtil::<&str>::error("默认权限无法修改")
    }
    let r = permissions::Entity::delete_by_id(permissions_id)
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
pub async fn permissions_update(
    State(db): State<DatabaseConnection>,
    Json(permissions_info): Json<permissions::Model>
) -> Json<Value>
{
    let mut permissions_info = permissions_info.into_active_model();
    permissions_info.permission_name = Set(permissions_info.permission_name.unwrap());
    permissions_info.description = Set(permissions_info.description.unwrap());
    let r = permissions_info.update(&db).await;
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
pub async fn permissions_select_all(
    State(db): State<DatabaseConnection>,
    Path(roles_id): Path<i64>
) -> Json<Value>
{
    let r = permissions::Entity::find().all(&db).await;
    match r {
        Ok(r) => {
            //创建临时集合
            let mut v: Vec<_> = Vec::new();
            //判断是否被角色绑定
            for temp in r {
                let r_temp = roles_permissions::Entity::find()
                    .filter(
                        Condition::all()
                            .add(
                                roles_permissions::Column::RolesId.eq(roles_id)
                            )
                            .add(
                                roles_permissions::Column::PermissionsId.eq(temp.id)
                            )
                    )
                    .one(&db)
                    .await;
                //赋值boolean类型用于判断是否绑定
                match r_temp {
                    Ok(r) => {
                        match r {
                            None => {v.push((temp, false))}
                            Some(_) => {v.push((temp, true))}
                        }
                    }
                    Err(_) => {
                        v.push((temp, false))
                    }
                }
            }
            ResultUtil::success(v)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}




///添加
pub async fn roles_insert(
    State(db): State<DatabaseConnection>,
    Json(roles_info): Json<roles::Model>
) -> Json<Value>
{
    let mut roles_info = roles_info.into_active_model();

    roles_info.id = NotSet;

    let r = roles_info.insert(&db).await;

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
pub async fn roles_delete(
    State(db): State<DatabaseConnection>,
    Path(roles_id): Path<i64>
) -> Json<Value>
{
    if roles_id == 0 {
        return ResultUtil::<&str>::error("默认权限无法修改")
    }
    let r = roles::Entity::delete_by_id(roles_id)
        .exec(&db)
        .await;
    //删除对应权限
    let _ = roles_permissions::Entity::delete_many()
        .filter(
            Condition::all()
                .add(
                    roles_permissions::Column::RolesId.eq(roles_id)
                )
        )
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
pub async fn roles_update(
    State(db): State<DatabaseConnection>,
    Json(roles_info): Json<roles::Model>
) -> Json<Value>
{
    let mut roles_info = roles_info.into_active_model();
    roles_info.role_name = Set(roles_info.role_name.unwrap());
    roles_info.description = Set(roles_info.description.unwrap());
    let r = roles_info.update(&db).await;
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
pub async fn roles_select_all(
    State(db): State<DatabaseConnection>
) -> Json<Value>
{
    let r = roles::Entity::find().all(&db).await;
    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}



///添加
pub async fn rp_insert(
    State(db): State<DatabaseConnection>,
    Json(rp_info): Json<roles_permissions::Model>
) -> Json<Value>
{
    if rp_info.permissions_id == Some(0) {
        return ResultUtil::<&str>::error("默认权限无法修改")
    }
    let mut rp_info = rp_info.into_active_model();

    rp_info.id = NotSet;
    rp_info.roles_id = Set(rp_info.roles_id.unwrap());
    rp_info.permissions_id = Set(rp_info.permissions_id.unwrap());

    let r = rp_info.insert(&db).await;

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
pub async fn rp_delete(
    State(db): State<DatabaseConnection>,
    Json(rp_info): Json<roles_permissions::Model>
) -> Json<Value>
{
    if rp_info.permissions_id == Some(0) {
        return ResultUtil::<&str>::error("默认权限无法修改")
    }
    let r = roles_permissions::Entity::delete_many()
        .filter(
            Condition::all()
                .add(
                    roles_permissions::Column::RolesId.eq(rp_info.roles_id)
                )
                .add(
                    roles_permissions::Column::PermissionsId.eq(rp_info.permissions_id)
                )
        )
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


///根据用户查角色
pub async fn select_user_roles(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>
) -> Json<Value>
{
    let r = user_roles::Entity::find()
        .filter(
            Condition::all()
                .add(
                    user_roles::Column::UserId.eq(user_id)
                )
        )
        .one(&db)
        .await;
    match r {
        Ok(r) => {
            match r {
                Some(r) => {
                    ResultUtil::success(r)
                }
                None => {
                    ResultUtil::<&str>::error("没有对应权限")
                }
            }
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}

///修改用户权限
pub async fn update_user_roles(
    State(db): State<DatabaseConnection>,
    Json(user_roles_info): Json<user_roles::Model>
) -> Json<Value>
{
    if user_roles_info.user_id == 0 {
        return ResultUtil::<&str>::error("默认权限无法修改")
    }
    let mut  user_roles_info = user_roles_info.into_active_model();
    user_roles_info.role_id = Set(user_roles_info.role_id.unwrap());
    user_roles_info.user_id = Set(user_roles_info.user_id.unwrap());

    let r = user_roles_info.update(&db).await;
    match r {
        Ok(_) => {
            ResultUtil::success(true)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}