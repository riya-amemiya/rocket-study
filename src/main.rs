use sea_orm::Database;
use std::env;

use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
mod routes;
use routes::*;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http,
    http::Header,
    serde::{json::Json, Serialize},
    Request, Response,
};

const DATABASE_URL: &str =
    "postgresql://postgres:fnWweFmVePXpHnEChqbdAMwBLUBUQrVZ@junction.proxy.rlwy.net:40615/railway";

#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
            get_uuids_v7_handler,
            calculator_index
        ),
        tags(
            (name = "index", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    let mut doc = ApiDoc::openapi();
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());
    let servers = [server_url];
    doc.servers = Some(
        servers
            .iter()
            .map(|x| utoipa::openapi::Server::new(x))
            .collect::<Vec<_>>(),
    );

    let db = Database::connect(DATABASE_URL).await.map_err(|e| {
        println!("Database connection error: {:?}", e);
        rocket::Error::from(rocket::error::ErrorKind::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Database connection error",
        )))
    })?;
    println!("{:?}", db);

    let _ = rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
                get_uuid_v4_handler,
                get_uuids_v4_handler,
                get_uuid_v7_handler,
                get_uuids_v7_handler,
                calculator_index
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", doc),
        )
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
