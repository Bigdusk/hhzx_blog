use axum::Extension;
use axum::extract::{Path, State};
use axum::response::Json;
use chrono::Local;
use log::info;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, IntoActiveModel, JsonValue, NotSet, sea_query, Statement};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::utils::result::ResultUtil;
use crate::entity::{article, user};
use crate::entity::tags;

///添加文章
pub async fn insert(
    Extension(current_user): Extension<user::Model>,
    State(db): State<DatabaseConnection>,
    Json(article): Json<article::Model>,
) -> Json<Value> {

    //创建文章
    let mut article = article.into_active_model();

    article.id = NotSet;
    article.user_id = Set(Some(current_user.id));
    article.publish_time = Set(Some(Local::now().naive_local()));
    article.update_time = Set(Some(Local::now().naive_local()));
    article.likes = Set(Some(0));
    article.views = Set(Some(0));

    //标签添加
    let tags = article.clone().article_tags
        .into_value()
        .unwrap_or(sea_query::value::Value::from("默认"))
        .to_string();
    let tags = &tags[1..tags.len() - 1];
    for tag_name in tags.split(",") {
        let mut tag_temp = tags::ActiveModel::new();
        tag_temp.id = NotSet;
        tag_temp.tag_name = Set(tag_name.to_string());
        tag_temp.updated_at = Set(Local::now().naive_local());
        tag_temp.created_at = Set(Local::now().naive_local());
        let _ = tag_temp.insert(&db).await;
    }

    let r = article.insert(&db).await;

    match r {
        Ok(_) => {
            ResultUtil::success(true)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}
///修改
pub async fn update(
    State(db): State<DatabaseConnection>,
    Json(article_info): Json<article::Model>
) -> Json<Value>
{
    let mut article_info = article_info.into_active_model();

    article_info.category_id = Set(article_info.category_id.unwrap());
    article_info.article_tags = Set(article_info.article_tags.unwrap());
    article_info.title = Set(article_info.title.unwrap());
    article_info.cover = Set(article_info.cover.unwrap());
    article_info.router = Set(article_info.router.unwrap());
    article_info.markdown = Set(article_info.markdown.unwrap());
    article_info.html = Set(article_info.html.unwrap());
    article_info.update_time = Set(Some(Local::now().naive_local()));
    article_info.status = Set(article_info.status.unwrap());

    //标签添加
    let tags = article_info.clone().article_tags
        .into_value()
        .unwrap_or(sea_query::value::Value::from("默认"))
        .to_string();
    let tags = &tags[1..tags.len() - 1];
    info!("{:?}", tags);
    for tag_name in tags.split(",") {
        let mut tag_temp = tags::ActiveModel::new();
        tag_temp.id = NotSet;
        tag_temp.tag_name = Set(tag_name.to_string());
        tag_temp.updated_at = Set(Local::now().naive_local());
        tag_temp.created_at = Set(Local::now().naive_local());
        let _ = tag_temp.insert(&db).await;
    }

    let r = article_info.update(&db).await;

    match r {
        Ok(_) => {
            ResultUtil::success(true)
        }
        Err(_) => {
            ResultUtil::<&str>::error("修改失败")
        }
    }
}
///删除
pub async fn delete(
    State(db): State<DatabaseConnection>,
    Path(article_id): Path<i64>
) -> Json<Value>
{
    let r = article::Entity::delete_by_id(article_id)
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

///根据id获取文章信息
pub async fn select_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<Value> {

    let r = article::Entity::find_by_id(id)
        .one(&db)
        .await;

    match r {
        Ok(r) => {
            match r {
                None => { ResultUtil::<&str>::error("没有该文章") }
                Some(r) => {
                    //这个不安全后期修复
                    let mut temp =r.clone().into_active_model();
                    temp.views = Set(Some(temp.views.unwrap().unwrap() + 1));
                    temp.update(&db).await.unwrap();
                    ResultUtil::success(r)
                }
            }
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}
///查询所有文章
pub async fn select_all(
    State(db): State<DatabaseConnection>
) -> Json<Value>
{
    let r = JsonValue::find_by_statement(
        Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
            SELECT
                article.id,
                user.nickname,
                category.category_name,
                article.title,
                article.cover,
                article.publish_time,
                article.update_time,
                article.status,
                article.views,
                article.likes
            FROM `article`, `user`,`category`
            WHERE article.user_id = user.id
            AND article.category_id = category.id
            ORDER BY article.id DESC
            "#,
            [],
        )
    )
        .all(&db)
        .await;

    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}
///分页，条件查询
pub async fn page_and_title(
    State(db): State<DatabaseConnection>,
    Path((page, size, value)): Path<(u64, u64, String)>,
) -> Json<Value> {
    let page = (page - 1) * size;
    let r;
    if value == "undefined" {
        r = JsonValue::find_by_statement(
            Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"
            SELECT
                article.id,
                user.nickname,
                category.category_name,
                article.title,
                article.cover,
                article.publish_time,
                article.update_time,
                article.status,
                article.views,
                article.likes
            FROM `article`, `user`,`category`
            WHERE article.user_id = user.id
            AND article.status = 0
            AND article.category_id = category.id
            ORDER BY article.id DESC
            LIMIT ?, ?
            "#,
                [page.into(), size.into()],
            )
        )
            .all(&db)
            .await;
    } else {
        let value = format!("%{value}%");
        r = JsonValue::find_by_statement(
            Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"
            SELECT
                article.id,
                user.nickname,
                category.category_name,
                article.title,
                article.cover,
                article.publish_time,
                article.update_time,
                article.status,
                article.views,
                article.likes
            FROM `article`, `user`,`category`
            WHERE article.user_id = user.id
            AND article.category_id = category.id
            AND article.status = 0
            AND article.title LIKE ?
            ORDER BY article.id DESC
            LIMIT ?, ?
            "#,
                [value.into(), page.into(), size.into()],
            )
        )
            .all(&db)
            .await;
    }


    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}

///分页，条件查询
pub async fn page_and_category(
    State(db): State<DatabaseConnection>,
    Path((page, size, id)): Path<(u64, u64, i64)>,
) -> Json<Value> {
    let page = (page - 1) * size;
    let r = JsonValue::find_by_statement(
        Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
            SELECT
                article.id,
                user.nickname,
                category.category_name,
                article.title,
                article.cover,
                article.publish_time,
                article.update_time,
                article.status,
                article.views,
                article.likes
            FROM `article`, `user`,`category`
            WHERE article.user_id = user.id
            AND article.category_id = category.id
            AND article.status = 0
            AND category.id = ?
            ORDER BY article.id DESC
            LIMIT ?, ?
            "#,
            [id.into(), page.into(), size.into()],
        )
    )
        .all(&db)
        .await;

    match r {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}