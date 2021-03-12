// Crates
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate redis;

// Modules
#[path = "./bin/routes/routes.rs"] pub mod routes;
#[path = "./bin/utils/db_connection.rs"] mod db_connection;
pub mod schema;
#[path = "./bin/models/models.rs"] pub mod models;

// Using
use crate::routes::user_routes;
use crate::routes::error_routes;


#[rocket::main]
async fn main(){

    let _ = rocket::ignite()
        .mount("/", routes![
            user_routes::create::user,
            user_routes::get_all::user,
            user_routes::get_id::user,
            user_routes::update::user,
            user_routes::delete::user,
        ])
        .register(catchers![
            error_routes::bad_request,
            error_routes::not_found,
            error_routes::internal_server_error,
        ])
        .attach(db_connection::DBConnect::fairing())
        .launch()
        .await;
}