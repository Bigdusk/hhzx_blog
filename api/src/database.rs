use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::utils::env_var;

pub async fn conn() -> DatabaseConnection {
    //生成实体
    //sea-orm-cli generate entity -u mysql://root:123456@127.0.0.1:3306/hhzx_blog

    let database_url: String = env_var("DATABASE_URL");

    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema");

    let db = Database::connect(opt).await.expect("数据连接失败");

    db
}