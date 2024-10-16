use rocket::{
    http,
    serde::{json::Json, uuid::Uuid, Serialize},
};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetUuidsV7Result {
    messages: Vec<String>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get UUID v7 result.", body = GetUuidsV7Result)
    )
)]
#[get("/getUuidsV7/<range>")]
pub fn get_uuids_v7_handler(range: i32) -> (http::Status, Json<GetUuidsV7Result>) {
    (
        http::Status::Ok,
        Json(GetUuidsV7Result {
            messages: (0..range).map(|_| format!("{}", Uuid::now_v7())).collect(),
        }),
    )
}
