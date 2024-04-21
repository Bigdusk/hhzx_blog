use axum::{middleware, Router};
use axum::routing::get;
use hhzx_cors::cors;
use hhzx_log::log;
use crate::controller::{
    index_controller
};
use crate::database;

///路由
pub async fn new() -> Router<()> {
    //获取数据库
    let db = database::conn().await;
    let app = Router::new()
        .route("/", get(index_controller::index))
        .route_layer(middleware::from_fn(log))
        .with_state(db)
        .route_layer(middleware::from_fn(cors));

    app
}