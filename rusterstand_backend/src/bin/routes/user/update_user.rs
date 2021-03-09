#[put("/user/<id>", format = "json")]
pub fn update_user(id: u32, conn: PgConnection) -> JsonValue{
    json!({
        "desc": "Hit Update User With Id",
        "id": id,
    })
}