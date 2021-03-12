// Main Components
use rocket_contrib::json::{ Json, JsonValue };
use diesel::prelude::*;
use crate::db_connection::DBConnect;

// User Components
use crate::schema::users::dsl::*;
use crate::models::*;

#[post("/user", format = "json", data="<new_user_data>")]
pub async fn user(conn: DBConnect, new_user_data: Json<new_user_model::NewUser>) -> JsonValue {
    conn.run(|c|{
        let res = diesel::insert_into(users)
            .values(new_user_data.into_inner())
            .execute(c)
            .expect("Error Creating New User in DB");
        
        json!(res)
    }).await
}

// #[post("/user", format = "json")]
// pub fn create_user(
//     first_name: String,
//     last_name: String,
//     email: String,
//     password_digest: String,
//     created_at: SystemTime
// ) -> user_model::User{
//     use crate::schema::users;
//     use crate::models::{user_model::User, new_user_model::NewUser};
//     let connection = db_connection::establish_connection();

//     let new_user = NewUser {
//         first_name:first_name,
//         last_name:last_name,
//         email:email,
//         password_digest:password_digest,
//         created_at: created_at,
//     };

//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .get_result(&connection)
//         .expect("Error Creating new user");

//     return user_model::User();
    
// }