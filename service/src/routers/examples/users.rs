use actix_web::{web, get, post, HttpResponse, Result};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String
}

#[get("/users")]
pub async fn index() -> Result<HttpResponse> {
    let user = User {
        name: String::from("Dummy User"),
        email: String::from("user@dummy.com")
    };

    // let user_json = serde_json::to_string(&user).unwrap();
    let res = json!({ "data": user });
    Ok(HttpResponse::Ok().json(handlers::formatter::success(res)))
}

#[derive(Deserialize)]
struct Info {
    keyword: String
}

#[get("/users/{user_id}")]
pub async fn show(user_id: web::Path<String>, query: Option<web::Query<Info>>) -> Result<HttpResponse> {
    let body = if query.is_none() {
        format!("User ID {}", user_id)
    } else {
        format!("User ID {}, Search: {}", user_id, query.unwrap().keyword)
    };

    // let is_query = query.is_none();
    // let body = match is_query {
    //     true => format!("User ID {}", user_id),
    //     false => format!("User ID {}, Search: {}", user_id, query.unwrap().keyword)
    // };

    let res = json!({ "data": body });
    Ok(HttpResponse::Ok().json(handlers::formatter::success(res)))
}

#[post("/users")]
pub async fn store(user: web::Json<User>) -> Result<HttpResponse> {
    let user = User {
        name: user.name.clone(),
        email: user.email.clone()
    };

    let res = json!({ "data": user });
    Ok(HttpResponse::Ok().json(handlers::formatter::success(res)))
}
