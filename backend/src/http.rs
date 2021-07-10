use std::convert::{Infallible, TryFrom};
use std::{net::SocketAddr, sync::Arc};

use http::{StatusCode, Uri};
use sqlx::PgPool;
use uuid::Uuid;
use warp::reply::Response;
use warp::{Filter, Future, Reply};

use crate::base62::{Base62, Error as Base62Error};
use crate::database::urls;
use crate::settings::HttpSettings;

#[tracing::instrument(name = "Handle getting a url by a slug", skip(db))]
pub async fn handle_get_url(slug: String, db: Arc<PgPool>) -> Result<Response, Response> {
    let base62 = Base62::try_from(slug.as_str()).map_err(|e| match e {
        Base62Error::IsEmpty => {
            tracing::info!("Empty slug was provided.");
            let reply = warp::reply::json(&"Slug cannot be empty.");
            warp::reply::with_status(reply, StatusCode::BAD_REQUEST).into_response()
        }
        Base62Error::InvalidChar(c) => {
            tracing::info!("Invalid char was detected {}", c);
            let reply = warp::reply::json(&format!("Slug contained invalid char of: '{}'", c));
            warp::reply::with_status(reply, StatusCode::BAD_REQUEST).into_response()
        }
    })?;

    let raw_url = urls::lookup_url(base62.into(), &db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                tracing::info!("The requested url was not found");
                warp::reply::with_status(warp::reply(), StatusCode::NOT_FOUND).into_response()
            }
            e => {
                tracing::error!("An unknown sql error occurred: {:?}", e);

                warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR)
                    .into_response()
            }
        })?;

    let uri = Uri::try_from(&raw_url).map_err(|_| {
        tracing::error!("An invalid url was pulled from the database: {}", raw_url);

        warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR).into_response()
    })?;

    tracing::info!("The url was found: {}", uri);
    Ok(warp::redirect(uri).into_response())
}

#[tracing::instrument(name = "Handle adding a url to the database", skip(db))]
pub async fn handle_post_url(url: String, db: Arc<PgPool>) -> Result<Response, Response> {
    let uri = Uri::try_from(&url).map_err(|_| {
        tracing::info!("An invalid url was provided.");
        let reply = warp::reply::json(&"URL is invalid");
        warp::reply::with_status(reply, StatusCode::BAD_REQUEST).into_response()
    })?;

    let key = urls::insert_url(&uri, &db).await.map_err(|e| {
        tracing::error!("An unknown sql error occurred: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let base62 = Base62::from(key);

    let reply =
        warp::reply::with_status(warp::reply::json(&base62.to_string()), StatusCode::CREATED)
            .into_response();

    Ok(reply)
}

fn create_socket_addr(settings: &HttpSettings) -> SocketAddr {
    let addr = format!("{}:{}", settings.host, settings.port);
    addr.parse().expect("Invalid host and/or port provided.")
}

fn with_data<T: Clone + Send>(data: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || data.clone())
}

async fn reply_try_future(
    fut: impl Future<Output = Result<Response, Response>>,
) -> Result<Response, Infallible> {
    match fut.await {
        Ok(r) => Ok(r),
        Err(r) => Ok(r),
    }
}

pub async fn start(
    settings: &HttpSettings,
    db_pool: PgPool,
) -> (SocketAddr, impl Future<Output = ()> + 'static) {
    let db = Arc::new(db_pool);

    let get_health_check = warp::path("health-check")
        .and(warp::get())
        .map(|| http::StatusCode::NO_CONTENT);

    let get_url = warp::get()
        .and(warp::path!(String))
        .and(with_data(db.clone()))
        .map(handle_get_url)
        .and_then(reply_try_future);

    let post_url = warp::post()
        .and(warp::body::json())
        .and(with_data(db))
        .map(handle_post_url)
        .and_then(reply_try_future);

    let filters = get_health_check
        .or(get_url)
        .or(post_url)
        .with(warp::filters::trace::request())
        .with(warp::filters::trace::trace(|_info| {
            let request_id = Uuid::new_v4();
            tracing::info_span!("request", id = ?request_id)
        }));

    warp::serve(filters).bind_ephemeral(create_socket_addr(settings))
}
