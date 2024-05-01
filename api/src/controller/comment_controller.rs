use axum::extract::{Path, State};
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::utils::result::ResultUtil;
use crate::entity::{comment, user};


pub async fn insert(
    State(db): State<DatabaseConnection>,
    Json(comment): Json<comment::Model>
) -> Json<Value> {

    let mut comment = comment.into_active_model();
    comment.id = NotSet;
    comment.create_time = Set(Some(Local::now().naive_local()));
    comment.update_time = Set(Some(Local::now().naive_local()));

    let result = comment.insert(&db).await;

    match result {
        Ok(_) => {
            ResultUtil::success(true)
        }
        Err(_) => {
            ResultUtil::<String>::error("添加失败")
        }
    }
}


pub async fn delete_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<Value> {

    let result = user::Entity::delete_by_id(id).exec(&db).await;

    match result {
        Ok(r) => {
            if r.rows_affected > 0 {
                ResultUtil::success(true)
            } else {
                ResultUtil::<String>::error("删除失败")
            }
        }
        Err(_) => {
            ResultUtil::<String>::error("删除失败")
        }
    }
}


pub async fn select_all(
    State(db): State<DatabaseConnection>
) -> Json<Value> {

    let result = comment::Entity::find().all(&db).await;

    match result {
        Ok(r) => {
            ResultUtil::success(r)
        }
        Err(_) => {
            ResultUtil::<String>::error("查询失败")
        }
    }
}

pub async fn select_all_by_article_id(
    State(db): State<DatabaseConnection>,
    Path(article_id): Path<i64>,
) -> Json<Value> {

    let result = comment::Entity::find()
        .filter(
            Condition::all()
                .add(
                    comment::Column::ArticleId.eq(article_id)
                )
                .add(
                    comment::Column::CommentId.is_null()
                )
        )
        .all(&db)
        .await;
    match result {
        Ok(r) => {
            //回复查询
            let mut comment_and_comment_list = Vec::new();

            //查询子回复
            for temp_comment in r {
                let r2 = comment::Entity::find()
                    .filter(Condition::all().add(
                        comment::Column::CommentId.eq(temp_comment.id)
                    )).all(&db).await.unwrap();
                comment_and_comment_list.push((temp_comment, r2))
            }

            ResultUtil::success(comment_and_comment_list)
        }
        Err(_) => {
            ResultUtil::<String>::error("查询失败")
        }
    }
}