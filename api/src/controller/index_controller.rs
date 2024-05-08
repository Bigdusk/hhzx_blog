use axum::Json;
use axum::extract::State;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use serde_json::Value;
use crate::entity::{article, category, tags};
use crate::utils::result::ResultUtil;

pub async fn index(
    _db: State<DatabaseConnection>
) -> Result<Json<Value>, String>{
    Ok(ResultUtil::success("欢迎使用hhzx_blog系统"))
}

///获取后台主页信息
pub async fn info_count(
    State(db): State<DatabaseConnection>,
) -> Json<Value>{
    let article_count = article::Entity::find()
        .count(&db)
        .await
        .unwrap();

    let category_count = category::Entity::find()
        .count(&db)
        .await
        .unwrap();

    let tags_count = tags::Entity::find()
        .count(&db)
        .await
        .unwrap();

    ResultUtil::success((article_count, category_count, tags_count))
}