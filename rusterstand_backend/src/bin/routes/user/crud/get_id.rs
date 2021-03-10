// Main Components
use rocket_contrib::json::JsonValue;

#[get("/user/<id>")]
pub fn user(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Get User By ID",
        "id": id
    })
}