#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;

#[path = "./Controller/_Controller.rs"] mod Controller;
#[path = "./Routes/_Routes.rs"] mod Routes;

#[rocket::main]
async fn main(){
    let _ = rocket::ignite()
        .mount("/", routes![
            Controller::UserController::get_all_user,
            Controller::UserController::get_user_by_id,
            Controller::UserController::create_user,
            Controller::UserController::update_user,
            Controller::UserController::delete_user,
        ])
        .register(catchers![
            Routes::Errors::bad_request,
            Routes::Errors::not_found,
            Routes::Errors::internal_server_error,
        ])
        .launch()
        .await;
}