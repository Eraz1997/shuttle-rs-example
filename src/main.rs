use axum::extract::Json;
use axum::extract::Path;
use axum::routing::post;
use axum::{routing::get, Extension, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct RequestPayload {
    url: String,
}

async fn redirect(Path(id): Path<String>, Extension(pool): Extension<PgPool>) -> &'static str {
    "Hello, world!"
}

async fn shorten_url(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RequestPayload>,
) -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/:id", get(redirect))
        .route("/", post(shorten_url))
        .layer(Extension(pool));

    Ok(router.into())
}
