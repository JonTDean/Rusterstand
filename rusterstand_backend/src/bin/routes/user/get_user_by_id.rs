#[get("/user/<id>")]
pub fn get_user_by_id(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Get User By ID",
        "id": id
    })
}