use rocket::{
    http,
    serde::{json::Json, uuid::Uuid, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetUuidV7Result {
    message: String,
}

#[get("/getUuidV7")]
pub fn get_uuid_v7_handler() -> (http::Status, Json<GetUuidV7Result>) {
    (
        http::Status::Ok,
        Json(GetUuidV7Result {
            message: format!("{}", Uuid::now_v7()),
        }),
    )
}
