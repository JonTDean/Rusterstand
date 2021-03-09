#[delete("/user/<id>", format = "json")]
pub fn delete_user(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Delete User With Id",
        "id": id
    })
}
