use axum::extract::{Path, State};
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet, Order, QueryFilter, QueryOrder, TransactionTrait};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
use crate::utils::result::ResultUtil;
use crate::entity::comment;


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
    //删除关联评论
    let deletes = comment::Entity::delete_many()
        .filter(
            Condition::all()
                .add(
                    comment::Column::CommentId.eq(id)
                )
        )
        .exec(&db)
        .await;
    match deletes {
        Ok(_r) => {}
        Err(_) => {return ResultUtil::<String>::error("删除关联评论失败")}
    }

    let r = comment_delete_by_id(db, id).await;
    match r {
        Ok(r) => {
            r
        }
        Err(r) => {
            ResultUtil::<String>::error(r.to_string().as_str())
        }
    }
}

async fn comment_delete_by_id(db: DatabaseConnection, id: i64) -> Result<Json<Value>, Box<dyn std::error::Error>>{
    let txn = db.begin().await?;
    comment::Entity::delete_by_id(id).exec(&txn).await?;
    comment::Entity::delete_many()
        .filter(
            comment::Column::CommentId.eq(id)
        )
        .exec(&txn)
        .await?;
    txn.commit().await?;

    Ok(ResultUtil::success(true))
}


pub async fn select_all(
    State(db): State<DatabaseConnection>
) -> Json<Value> {

    let result = comment::Entity::find().order_by(comment::Column::Id, Order::Desc).all(&db).await;

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