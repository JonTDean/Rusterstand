// Main Components
use rocket_contrib::json::JsonValue;
use diesel::prelude::*;
use crate::db_connection::DBConnect;

// User Components
use crate::schema::users::dsl::*;
use crate::models::*;

#[get("/users")]
pub async fn user(conn: DBConnect) -> JsonValue{
    // Establishes a Postgres Connection and drops
    // it when it's done
    conn.run( |connection| {
        // Sends an outgoing request to the postgres connection
        // and loads the table from the specified database
        let res = users
            .limit(25)
            .load::<user_model::User>(connection)
            .expect("Error loading Users");


        // Checks if there are any Users in the db.
        // Errors out if is empty,
        // Returns an entire json of Response from Users Vector
        if res.is_empty() {
            println!("Users Table is Empty!");
            println!("Please add a User to the Table");
            json!({
                "err": [
                    "Users Table is Empty!",
                    "Please add a User to the Table"
                ]
            })
        }else {
            println!("Displaying {} Users", res.len());

            for user in &res{
                println!("User ID:{}", user.id);
                println!("User First Name:{}", user.first_name);
                println!("User Last Name:{}", user.last_name);
                println!("User E-Mail:{}", user.email);
            }
            
            json!(&res)
        }
    }).await
}