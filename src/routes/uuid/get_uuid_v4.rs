use rocket::{
    http,
    serde::{json::Json, uuid::Uuid, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetUuidV4Result {
    message: String,
}

#[get("/getUuidV4")]
pub fn get_uuid_v4_handler() -> (http::Status, Json<GetUuidV4Result>) {
    (
        http::Status::Ok,
        Json(GetUuidV4Result {
            message: format!("{}", Uuid::new_v4()),
        }),
    )
}
