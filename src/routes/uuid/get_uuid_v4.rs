use rocket::{
    http,
    serde::{json::Json, uuid::Uuid, Serialize},
};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetUuidV4Result {
    message: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get UUID v4 result.", body = [GetUuidV4Result])
    )
)]
#[get("/getUuidV4")]
pub fn get_uuid_v4_handler() -> (http::Status, Json<GetUuidV4Result>) {
    (
        http::Status::Ok,
        Json(GetUuidV4Result {
            message: format!("{}", Uuid::new_v4()),
        }),
    )
}
