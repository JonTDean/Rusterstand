
use rocket_contrib::json::{ Json, JsonValue };
use crate::models::new_user_model;

#[post("/user", format = "json", data="<new_user_data>")]
pub async fn create_user(new_user_data: Json<new_user_model::NewUser>) -> JsonValue {
    json!("LOL")
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