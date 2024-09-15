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
    let data = IndexResult {
        message: format!("hello world!"),
    };
    (http::Status::Ok, Json(data))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
