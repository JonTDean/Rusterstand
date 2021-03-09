// Crates
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

// Modules
#[path = "./bin/controller/user_controller.rs"] mod user_controller;
// #[path = "./bin/routes/user/user_routes.rs"] pub mod user_routes;
#[path = "./bin/routes/routes.rs"] pub mod routes;
#[path = "./bin/utils/db_connection.rs"] mod db_connection;
pub mod schema;
#[path = "./bin/models/models.rs"] pub mod models;

// Using
use crate::routes::;

#[rocket::main]
async fn main(){

    let _ = rocket::ignite()
        .mount("/", routes![
            user_routes::create_user::create_user,
            user_routes::get_user_by_id::get_user_by_id,
            user_routes::get_all_users::get_all_users,
            user_routes::update_user::update_user,
            user_routes::delete_user::delete_user,
        ])
        .register(catchers![
            routes::errors::bad_request,
            routes::errors::not_found,
            routes::errors::internal_server_error,
        ])
        .launch()
        .await;
}