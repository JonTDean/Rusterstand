use crate::schema::users;
use std::time::SystemTime;
use serde::{Serialize, Deserialize };

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_digest: String,
    pub created_at: SystemTime,
}