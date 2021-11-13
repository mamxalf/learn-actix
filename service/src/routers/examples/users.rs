use actix_web::{get, web, HttpResponse, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
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
    Ok(HttpResponse::Ok().json(user))
}

#[derive(Deserialize)]
struct Info {
    keyword: String
}

#[get("/users/{user_id}")]
pub async fn show(user_id: web::Path<String>, query: Option<web::Query<Info>>) -> Result<String> {
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

    Ok(body)
}
