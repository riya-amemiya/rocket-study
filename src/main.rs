mod routes;
use routes::*;

use rocket::{
    http,
    serde::{json::Json, Serialize},
    Request,
};

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct NotFound {
    message: String,
    debug: String,
}

#[catch(404)]
fn not_found(req: &Request) -> Json<NotFound> {
    Json(NotFound {
        message: "Not found".to_string(),
        debug: format!("{}", req.uri()),
    })
}

#[derive(Serialize)]
struct IndexResult {
    message: String,
}

#[get("/")]
fn index() -> (http::Status, Json<IndexResult>) {
    (
        http::Status::Ok,
        Json(IndexResult {
            message: "hello world!".to_string(),
        }),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, get_uuid_v4_handler, get_uuids_v4_handler],
        )
        .register("/", catchers![not_found])
}
