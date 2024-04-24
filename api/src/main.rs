use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use api::router;
use api::utils::env_var;

#[tokio::main]
async fn main() {
    //日志
    tracing_subscriber::fmt::init();
    //检查是否有配置文件
    dotenv::dotenv().expect("读取配置文件失败(请在根目录创建.env)");
    let port: u16 = env_var("PORT");
    //读取路由
    let app = router::new().await;
    //读取ssl协议证书
    let config = RustlsConfig::from_pem_file(
        "self-signed-certs/cert.pem",
        "self-signed-certs/key.pem",
    )
        .await
        .expect("ssl证书协议读取失败");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Run server https://127.0.0.1:{}", addr.port());

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

