use rocket::Request;
use rocket_contrib::json::JsonValue;

// Error 400 Codes
// Bad Request
// The server could not understand the request due to invalid syntax.
#[catch(400)]
pub fn bad_request(req: &Request) -> JsonValue {
    json!({
        "Desc": "Couldn't find Supplied Link, try another Link.",
        "link": format!("I couldn't find '{}'. Try something else?", req.uri()),
    })
}

// Not Found
// The server can not find the requested resource. In the browser,
// this means the URL is not recognized. In an API, this can also
// mean that the endpoint is valid but the resource itself does not exist. 
#[catch(404)]
pub fn not_found() -> JsonValue {
    json!("Not Found!")
}

// Error 500 Codes
// Internal Server Error, server wigs out.
#[catch(500)]
pub fn internal_server_error() -> JsonValue {
    json!("Whoops! Looks like we messed up.")
}
