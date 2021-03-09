use rocket_contrib::json::JsonValue;

#[get("/users")]
pub fn get_all_users() -> JsonValue{
    // use crate::schema::users::dsl::*;

    // db_connection::establish_connection().run(|c| {
    //     let res = users
    //         .load::<user_model::User>(c)
    //         .expect("Error Loading Users");
    
    //     if res.len() < 1{
    //         println!("Users Table is Empty!");
    //         println!("Please add a User to the Table");
    //         json!({
    //             "err": [
    //                 "Users Table is Empty!",
    //                 "Please add a User to the Table"
    //             ]
    //         })
    //     }else {
    //         println!("Displaying {} Users", res.len());
    
    //         for user in &res{
    //             println!("User ID:{}", user.id);
    //             println!("User First Name:{}", user.first_name);
    //             println!("User Last Name:{}", user.last_name);
    //             println!("User E-Mail:{}", user.email);
    //         }
            
    //         json!(&res)
    //     }
    // });

    json!("trash")
}