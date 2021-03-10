// Main Components
use rocket_contrib::json::JsonValue;

#[delete("/user/<id>", format = "json")]
pub fn user(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Delete User With Id",
        "id": id
    })
}
