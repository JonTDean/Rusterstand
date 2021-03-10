// Main Components
use rocket_contrib::json::JsonValue;

#[put("/user/<id>", format = "json")]
pub fn user(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Update User With Id",
        "id": id,
    })
}