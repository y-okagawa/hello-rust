use actix_web::{
    web,
    http::header::ContentType,
    App,
    HttpResponse,
    HttpServer,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserQuery {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct UserResponse {
    user: String,
}

#[actix_web::get("/users")]
async fn users(
    query: web::Query<UserQuery>,
) -> HttpResponse {
    let query = query.into_inner();
    let message = format!(
        "name: {}. age: {}",
        query.name, query.age
    );
    let body = UserResponse { user: message };
    HttpResponse::Ok().json(body)
}

#[actix_web::get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(ContentType::json())
        .body(r#"{"hello":"1234"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(hello)
        .service(users)
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
