use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
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
#[serde(crate = "rocket::serde")]
struct DebugInfo {
    method: String,
    uri: String,
    headers: Vec<(String, String)>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NotFound {
    message: String,
    debug: DebugInfo,
}

#[catch(404)]
fn not_found(req: &Request) -> Json<NotFound> {
    let headers = req
        .headers()
        .iter()
        .map(|h| (h.name().to_string(), h.value().to_string()))
        .collect();
    Json(NotFound {
        message: "Not found".to_string(),
        debug: DebugInfo {
            method: req.method().as_str().to_string(),
            uri: req.uri().path().to_string(),
            headers,
        },
    })
}

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
struct IndexResult {
    message: String,
}

#[utoipa::path(
        context_path = "",
        responses(
            (status = 200, description = "Index result.", body = [IndexResult])
        )
)]
#[get("/")]
fn index() -> (http::Status, Json<IndexResult>) {
    (
        http::Status::Ok,
        Json(IndexResult {
            message: "hello world!".to_string(),
        }),
    )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            index,
            get_uuid_v4_handler,
            get_uuids_v4_handler,
            get_uuid_v7_handler,
            get_uuids_v7_handler
        ),
        components(
            schemas(IndexResult,GetUuidV4Result,GetUuidsV4Result,GetUuidV7Result,GetUuidsV7Result)
        ),
        tags(
            (name = "index", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_uuid_v4_handler,
                get_uuids_v4_handler,
                get_uuid_v7_handler,
                get_uuids_v7_handler
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
