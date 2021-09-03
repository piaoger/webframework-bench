use axum::{
    //extract::{Extension, Json },
    //prelude::*,
    response::IntoResponse,
    AddExtensionLayer,
     handler::{get},
    Router,
    Json,
};
use http::{Response, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    id: i32,
    user_name: &'a str,
    wechat: &'a str,
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn user() -> impl IntoResponse {
    let u = User {
        id: 3,
        user_name: "axum567890",
        wechat: "mywechat_03",
    };
     Json(u)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/hello", get(hello)).route("/user", get(user));

    // Run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8083));

    println!("axum(rust) - http://127.0.0.1:{}", 8083);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
