use crate::base62::{Base62, Error as Base62Error};

use axum::{
    extract,
    response::{self, IntoResponse},
    Router,
};
use http::{StatusCode, Uri};
use sqlx::PgPool;

async fn lookup_url(key: u64, db: &PgPool) -> Result<String, sqlx::Error> {
    let record = sqlx::query!(
        r#" 
            SELECT u.url
            FROM public.urls AS u
            WHERE u.key = $1;
        "#,
        key as i64
    )
    .fetch_one(db)
    .await?;

    Ok(record.url)
}

async fn insert_url(uri: &Uri, db: &PgPool) -> Result<u64, sqlx::Error> {
    let url = uri.to_string();

    sqlx::query!(
        r#"
            INSERT INTO public.urls
            (url)
            SELECT $1
            WHERE NOT EXISTS (SELECT 'x' FROM public.urls WHERE url = $1)
            ON CONFLICT DO NOTHING;
        "#,
        &url
    )
    .execute(db)
    .await?;

    let record = sqlx::query!(
        r#"
            SELECT u.key
            FROM public.urls AS u
            WHERE u.url = $1;
        "#,
        &url
    )
    .fetch_one(db)
    .await?;

    Ok(record.key as u64)
}

async fn handle_get(
    slug: extract::Path<String>,
    db_pool: PgPool,
) -> Result<response::Redirect, (StatusCode, impl IntoResponse)> {
    let base62 = Base62::try_from(slug.0.as_str()).map_err(|e| match e {
        Base62Error::IsEmpty => {
            tracing::info!("Empty slug was provided.");
            (
                StatusCode::BAD_REQUEST,
                response::Json("Slug cannot be empty.".into()),
            )
        }
        Base62Error::InvalidChar(c) => {
            tracing::info!("Invalid char was detected {}", c);
            (
                StatusCode::BAD_REQUEST,
                response::Json(format!("Slug contained invalid char of: '{}'", c)),
            )
        }
    })?;

    let raw_url = lookup_url(base62.into(), &db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                tracing::info!("The requested url was not found");
                (
                    StatusCode::NOT_FOUND,
                    response::Json("No clue what this is?".into()),
                )
            }
            e => {
                tracing::error!("An unknown sql error occurred: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    response::Json("An internal server error was encountered.".into()),
                )
            }
        })?;

    let uri = Uri::try_from(&raw_url).map_err(|_| {
        tracing::error!("An invalid url was pulled from the database: {}", raw_url);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            response::Json("An internal server error was encountered.".into()),
        )
    })?;

    Ok(response::Redirect::permanent(&uri.to_string()))
}

async fn handle_post(
    url: extract::Json<String>,
    db_pool: PgPool,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {
    let uri = Uri::try_from(&url.0).map_err(|_| {
        tracing::trace!("An invalid url was provided.");
        (StatusCode::BAD_REQUEST, response::Json("URL is invalid."))
    })?;

    let key = insert_url(&uri, &db_pool).await.map_err(|e| {
        tracing::error!("An unknown sql error occurred: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            response::Json("An internal server error was encountered."),
        )
    })?;

    let base62 = Base62::from(key);

    Ok((StatusCode::CREATED, response::Json(base62.to_string())))
}

pub fn router(db_pool: PgPool) -> Router {
    use axum::routing::{get, post};

    Router::new()
        .route(
            "/:slug",
            get({
                let db_pool = db_pool.clone();
                move |slug| handle_get(slug, db_pool)
            }),
        )
        .route(
            "/",
            post({
                let db_pool = db_pool.clone();
                move |body| handle_post(body, db_pool)
            }),
        )
}
