use rocket::{
    http,
    serde::{json::Json, uuid::Uuid, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetUuidsV4Result {
    messages: Vec<String>,
}

#[get("/getUuidsV4/<range>")]
pub fn get_uuids_v4_handler(range: i32) -> (http::Status, Json<GetUuidsV4Result>) {
    (
        http::Status::Ok,
        Json(GetUuidsV4Result {
            messages: (0..range).map(|_| format!("{}", Uuid::new_v4())).collect(),
        }),
    )
}
