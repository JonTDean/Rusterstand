use std::time::SystemTime;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_digest: String,
    pub created_at: SystemTime,
}
