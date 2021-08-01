//! Example showing how to convert errors into responses and how one might do
//! dependency injection using trait objects.

#![allow(dead_code)]

use axum::{
    extract::{Extension, Json, UrlParams},
    prelude::*,
    response::IntoResponse,
    AddExtensionLayer,
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

async fn hello() -> impl IntoResponse {
    "Hello, World!"
}

async fn user() -> impl IntoResponse {
    let u = User {
        id: 1,
        user_name: "axum567890",
        wechat: "mywechat",
    };
    response::Json(u)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = route("/user", get(user)).route("/hello", get(hello));

    // Run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], 8083));

    println!("axum(rust) - http://127.0.0.1:{}", 8083);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
