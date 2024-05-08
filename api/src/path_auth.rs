use axum::{
    http::StatusCode,
    response::Response,
    middleware::Next,
    extract::Request,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use crate::database;
use crate::entity::{user, user_roles, roles, roles_permissions, permissions};
use crate::utils::jwt;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());



    //是否拦截路径
    if passage_is_permitted(req.uri().path()) {
        return Ok(next.run(req).await)
    }

    let auth_token = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let db = database::conn().await;

    let jwt_data = jwt::decode_token(auth_token);
    let jwt_data = match jwt_data {
        Ok(r) => {r}
        Err(_) => {return Err(StatusCode::UNAUTHORIZED);}
    };
    let user_id = jwt_data.claims.get_user_id();

    let r = user::Entity::find_by_id(user_id).one(&db).await.unwrap();

    match r {
        None => {
            Err(StatusCode::UNAUTHORIZED)
        }
        Some(current_user) => {
            req.extensions_mut().insert(current_user.clone());
            //判断当前用户是否拥有权限路径

            //获取用户角色关联表
            let current_user_roles = user_roles::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            user_roles::Column::UserId.eq(current_user.id.clone())
                        )
                )
                .one(&db)
                .await;
            let current_user_roles = match current_user_roles {
                Ok(r) => {r}
                Err(_) => {return Err(StatusCode::UNAUTHORIZED);}
            };
            let current_user_roles = match current_user_roles {
                None => {return Err(StatusCode::UNAUTHORIZED);}
                Some(r) => {r}
            };

            //获取角色
            let current_roles = roles::Entity::find_by_id(current_user_roles.role_id)
                .one(&db)
                .await;
            let current_roles = match current_roles {
                Ok(r) => {r}
                Err(_) => {return Err(StatusCode::UNAUTHORIZED);}
            };
            let current_roles = match current_roles {
                None => {return Err(StatusCode::UNAUTHORIZED);}
                Some(r) => {r}
            };

            //获取权限关联表
            let current_roles_permissions = roles_permissions::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            roles_permissions::Column::RolesId.eq(current_roles.id)
                        )
                )
                .all(&db)
                .await;
            let current_roles_permissions = match current_roles_permissions {
                Ok(r) => {r}
                Err(_) => {return Err(StatusCode::UNAUTHORIZED);}
            };

            //查询所有权限
            let current_user_permissions= {
                let mut temp: Vec<_> = Vec::new();

                for current_roles_permission in current_roles_permissions {
                    let r = permissions::Entity::find()
                        .filter(
                            Condition::all()
                                .add(
                                    permissions::Column::Id.eq(current_roles_permission.permissions_id)
                                )
                        )
                        .one(&db)
                        .await;
                    let r = match r {
                        Ok(r) => {r}
                        Err(_) => {return Err(StatusCode::UNAUTHORIZED);}
                    };
                    let r = match r {
                        None => {return Err(StatusCode::UNAUTHORIZED);}
                        Some(r) => {r}
                    };

                    temp.push(r);
                }

                temp
            };

            //当前地址遍历对比权限地址
            for permission in current_user_permissions {
                //如果有*全部权限直接返回
                if permission.clone().description.unwrap_or("没有权限路径内容".to_string()).eq("*") {
                    return Ok(next.run(req).await)
                }

                if req.uri().path().starts_with(&permission.description.unwrap_or("没有权限路径内容".to_string())) {
                    return Ok(next.run(req).await)
                }
            }
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}


fn passage_is_permitted(url: &str) -> bool{
    for allow_path in allow_path_all() {
        if url.starts_with(allow_path) {
            return true
        }
    }
    false
}

///允许访问路径
fn allow_path_all() -> Vec<&'static str> {
    vec![
        "/permission/select",
        "/config/update",
        "/config/select",
        "/article/like",
        "/tag/select",
        "/comment/select",
        "/comment/insert",
        "/category/select",
        "/article/select",
        "/register",
        "/login",
        "/file/download/"
    ]
}

//处理器参数获取
/*async fn handler(
    // extract the current user, set by the middleware
    Extension(current_user): Extension<user::Model>,
) {
    // ...
}*/
