use serde::Serialize;
use std::convert::Infallible;
use warp::Filter;

#[derive(Serialize)]
struct User<'a> {
    id: i32,
    user_name: &'a str,
    wechat: &'a str,
}

async fn query_userinfo() -> Result<impl warp::Reply, Infallible> {
    let user = User {
        id: 1,
        user_name: "warp567890",
        wechat: "mywechat_01",
    };
    Ok(warp::reply::json(&user))
}

/// filter: /hello
fn hello() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello")
        .and(warp::get())
        .map(|| "Hello, World!")
}

/// filter: /user
fn user() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::get())
        .and_then(query_userinfo)
}

fn apis() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello().or(user())
}

#[tokio::main]
async fn main() {
    let port = 8081;
    println!("warp(rust) - http://127.0.0.1:{}", port);

    let routes = apis();
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
