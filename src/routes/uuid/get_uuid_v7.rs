use rocket::{
    http,
    serde::{json::Json, uuid::Uuid, Serialize},
};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetUuidV7Result {
    message: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get UUID v7 array result.", body = GetUuidV7Result)
    )
)]
#[get("/getUuidV7")]
pub fn get_uuid_v7_handler() -> (http::Status, Json<GetUuidV7Result>) {
    (
        http::Status::Ok,
        Json(GetUuidV7Result {
            message: format!("{}", Uuid::now_v7()),
        }),
    )
}
