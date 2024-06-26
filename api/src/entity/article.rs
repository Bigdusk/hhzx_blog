//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub category_id: Option<i64>,
    pub article_tags: Option<String>,
    pub title: String,
    pub cover: Option<String>,
    pub router: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub markdown: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub html: Option<String>,
    pub publish_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status: i8,
    pub views: Option<i64>,
    pub likes: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
