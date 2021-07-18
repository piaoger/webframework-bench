use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::Serialize;

#[derive(Serialize)]
struct User<'a> {
    id: i32,
    user_name: &'a str,
    wechat: &'a str,
}

#[get("/hello")]
async fn hello(_info: web::Path<()>) -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/user")]
async fn user(_info: web::Path<()>) -> impl Responder {
    let user = User {
        id: 1,
        user_name: "myname",
        wechat: "mywechat",
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
