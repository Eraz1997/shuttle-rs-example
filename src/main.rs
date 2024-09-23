use axum::extract::{Json, Path};
use axum::http::header::HOST;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Redirect;
use axum::routing::post;
use axum::{routing::get, Extension, Router};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{Error, PgPool};
use url::Url;

#[derive(Serialize, Deserialize)]
struct RequestPayload {
    url: String,
}

async fn redirect(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Redirect, StatusCode> {
    let (url,): (String,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    Ok(Redirect::to(&url))
}

async fn shorten_url(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<RequestPayload>,
) -> Result<String, StatusCode> {
    let host = headers
        .get(HOST)
        .and_then(|header_value| header_value.to_str().ok())
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let id = &nanoid!(6);
    let url = Url::parse(&payload.url).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    sqlx::query("INSERT INTO urls(id, url) VALUES ($1, $2)")
        .bind(id)
        .bind(url.to_string())
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(format!("https://{host}/{id}"))
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("migrations/db")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .route("/:id", get(redirect))
        .route("/", post(shorten_url))
        .layer(Extension(pool));

    Ok(router.into())
}
