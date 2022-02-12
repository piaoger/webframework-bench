use poem::{get, handler, listener::TcpListener, web::Json, Route, Server};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    id: i32,
    user_name: &'a str,
    wechat: &'a str,
}

#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[handler]
fn user() -> Json<User<'static>> {
    let u = User {
        id: 3,
        user_name: "poem567890",
        wechat: "mywechat_04",
    };
    Json(u)
}

#[tokio::main]
async fn main() {
    let app = Route::new().at("/hello", get(hello)).at("/user", get(user));

    println!("poem(rust) - http://127.0.0.1:{}", 8084);
    let listener = TcpListener::bind("127.0.0.1:8084");
    let server = Server::new(listener);
    server.run(app).await.unwrap();
}
