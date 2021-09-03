use serde::{Deserialize, Serialize};
use poem::{handler, route, web::Path, route::get, Server,web::Json, Result};

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    id: i32,
    user_name: &'a str,
    wechat: &'a str,
}

#[handler]
fn hello(Path(name): Path<String>) -> &'static str {
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
    let app = route().at("/hello", get(hello)).at("/user", get(user));

    println!("poem(rust) - http://127.0.0.1:{}", 8084);
    let server = Server::bind("127.0.0.1:8084").await.unwrap();
    server.run(app).await.unwrap();
}
