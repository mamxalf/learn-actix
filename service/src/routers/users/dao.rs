use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct Info {
    pub keyword: String
}