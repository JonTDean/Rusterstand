use rocket_contrib::json::JsonValue;

#[get("/user")]
pub fn get_all_user() -> JsonValue{
    json!([
        {
            "id": 1,
            "desc": "Hit Get All Users"},
        { 
            "id": 2,
            "desc": "Hit Get All Users"
        }
    ])
}

#[get("/user/<id>")]
pub fn get_user_by_id(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Get User By ID",
        "id": id
    })
}

#[post("/user", format = "json")]
pub fn create_user() -> JsonValue{
    json!({"desc": "Hit Create User"})
}

#[put("/user/<id>", format = "json")]
pub fn update_user(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Update User With Id",
        "id": id,
    })
}

#[delete("/user/<id>", format = "json")]
pub fn delete_user(id: u32) -> JsonValue{
    json!({
        "desc": "Hit Delete User With Id",
        "id": id
    })
}
