use std::time::Duration;
use axum::{middleware, Router};
use axum::http::Method;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use hhzx_log::log;
use crate::controller::{
    index_controller,
    article_controller,
    login_controller,
    category_controller,
    user_controller,
    file_upload_controller,
    comment_controller,
    tag_controller,
    permissions_controller,
    timeline_controller
};
use crate::database;
use tower_http::cors::{CorsLayer, Any};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use crate::path_auth::auth;

///路由
pub async fn new() -> Router<()> {
    //获取数据库
    let db = database::conn().await;

    let app = Router::new()
        .route("/login", post(login_controller::login))
        .route("/registration", post(login_controller::registration))
        .route("/article/insert", post(article_controller::insert))
        .route("/article/select/:id", get(article_controller::select_by_id))
        .route("/article/select/:page/:size/title/:value", get(article_controller::page_and_title))
        .route("/article/select/:page/:size/category/:id", get(article_controller::page_and_category))
        .route("/article/select/all", get(article_controller::select_all))
        .route("/article/delete/:article_id", get(article_controller::delete))
        .route("/article/update", post(article_controller::update))

        .route("/category/insert", post(category_controller::insert))
        .route("/category/delete/:id", get(category_controller::delete))
        .route("/category/update", post(category_controller::update))
        .route("/category/select/all", get(category_controller::select_all))

        .route("/permissions/insert", post(permissions_controller::permissions_insert))
        .route("/permissions/delete/:id", get(permissions_controller::permissions_delete))
        .route("/permissions/update", post(permissions_controller::permissions_update))
        .route("/permissions/select/roles/:roles_id", get(permissions_controller::permissions_select_all))

        .route("/rp/insert", post(permissions_controller::rp_insert))
        .route("/rp/delete", post(permissions_controller::rp_delete))

        .route("/timeline/insert", post(timeline_controller::timeline_insert))
        .route("/timeline/select/all", get(timeline_controller::timeline_select_all))

        .route("/user/roles/select/:user_id", get(permissions_controller::select_user_roles))
        .route("/user/roles/update", post(permissions_controller::update_user_roles))

        .route("/roles/insert", post(permissions_controller::roles_insert))
        .route("/roles/delete/:id", get(permissions_controller::roles_delete))
        .route("/roles/update", post(permissions_controller::roles_update))
        .route("/roles/select/all", get(permissions_controller::roles_select_all))

        .route("/user/login/info", get(user_controller::login_user_info))
        .route("/user/delete/:id", get(user_controller::delete))
        .route("/user/update", post(user_controller::update))
        .route("/user/select/all", get(user_controller::select_all))

        .route("/comment/insert", post(comment_controller::insert))
        .route("/comment/delete/:id", get(comment_controller::delete_by_id))
        .route("/comment/select/all", get(comment_controller::select_all))
        .route("/comment/select/article/:id", get(comment_controller::select_all_by_article_id))

        .route("/tag/update", post(tag_controller::update))
        .route("/tag/delete/:tag_id", get(tag_controller::delete))
        .route("/tag/select/all", get(tag_controller::select_all))
        .route("/tag/select/:page/:size", get(tag_controller::select_all_page))

        .route("/info_count", get(index_controller::info_count))
        .route("/", get(index_controller::index))
        .route("/file/upload", post(file_upload_controller::file_upload))
        .route("/file/download/:file_name", get(file_upload_controller::file_download))
        .route_layer(middleware::from_fn(log))
        .route_layer(middleware::from_fn(auth))
        .with_state(db)

        .layer(
            // 官方推荐在ServiceBuilder上一次性载入
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                // 超时时间是1s200ms
                .layer(TimeoutLayer::new(Duration::new(3, 200000)))
                // 默认情况下不放行，所以需要根据自己需求设置必要的允许规则。
                .layer(
                    CorsLayer::permissive()
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::DELETE,
                            Method::PUT,
                            Method::OPTIONS
                        ])
                        .allow_origin(Any)
                )

        );

    app
}