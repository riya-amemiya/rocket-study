use rocket::{
    http,
    serde::{json::Json, Serialize},
};

#[derive(Serialize)]
struct IndexResult {
    message: String,
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> (http::Status, Json<IndexResult>) {
    (
        http::Status::Ok,
        Json(IndexResult {
            message: format!("hello world!"),
        }),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
