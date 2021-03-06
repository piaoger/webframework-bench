use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    id: i32,
    user_name: &'a str,
    wechat: &'a str,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/user")]
async fn user() -> impl Responder {
    let user = User {
        id: 2,
        user_name: "actix67890",
        wechat: "mywechat_02",
    };
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = "8082";
    let addr = format!("127.0.0.1:{}", port);
    println!("actix(rust) - http://{}", addr);

    HttpServer::new(|| App::new().service(user).service(hello))
        .bind(addr)?
        .run()
        .await
}
