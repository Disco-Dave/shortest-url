use std::convert::Infallible;
use std::{net::SocketAddr, sync::Arc};

use sqlx::PgPool;
use uuid::Uuid;
use warp::reply::Response;
use warp::{Filter, Future, Reply};

use crate::actions;
use crate::settings::HttpSettings;

#[tracing::instrument(name = "Handle getting a url by a slug", skip(db))]
pub async fn handle_get_url(slug: String, db: Arc<PgPool>) -> Result<Response, Infallible> {
    use actions::GetUrlError::*;

    let response = match actions::get_url(slug, &db).await {
        Err(SlugIsEmpty) => {
            tracing::info!("Empty slug was provided.");
            let reply = warp::reply::json(&"Slug cannot be empty.");
            warp::reply::with_status(reply, http::StatusCode::BAD_REQUEST).into_response()
        }
        Err(SlugContaintsInvalidChar(c)) => {
            tracing::info!("Invalid char was detected {}", c);
            let reply = warp::reply::json(&format!("Slug contained invalid char of: '{}'", c));
            warp::reply::with_status(reply, http::StatusCode::BAD_REQUEST).into_response()
        }
        Err(UrlNotFound) => {
            tracing::info!("The requested url was not found");
            warp::reply::with_status(warp::reply(), http::StatusCode::NOT_FOUND).into_response()
        }
        Err(InvalidUrl(url)) => {
            tracing::error!("An invalid url was pulled from the database: {}", url);

            warp::reply::with_status(warp::reply(), http::StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
        Err(SqlError(e)) => {
            tracing::error!("An unknown sql error occurred: {:?}", e);

            warp::reply::with_status(warp::reply(), http::StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
        Ok(uri) => {
            tracing::info!("The url was found: {}", uri);

            warp::redirect(uri).into_response()
        }
    };

    Ok(response)
}

#[tracing::instrument(name = "Handle adding a url to the database", skip(db))]
pub async fn handle_post_url(url: String, db: Arc<PgPool>) -> Result<Response, Infallible> {
    use actions::PostUrlError::*;

    let response = match actions::post_url(url, &db).await {
        Err(InvalidUrl) => {
            tracing::info!("An invalid url was provided.");

            let reply = warp::reply::json(&"URL is invalid");
            warp::reply::with_status(reply, http::StatusCode::BAD_REQUEST).into_response()
        }
        Err(SqlError(e)) => {
            tracing::error!("An unknown sql error occurred: {:?}", e);

            warp::reply::with_status(warp::reply(), http::StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
        Ok(base62) => {
            tracing::info!("A url was created with the following slug: {}", base62);

            let reply = warp::reply::json(&base62.to_string());
            warp::reply::with_status(reply, http::StatusCode::CREATED).into_response()
        }
    };

    Ok(response)
}

fn create_socket_addr(settings: &HttpSettings) -> SocketAddr {
    let addr = format!("{}:{}", settings.host, settings.port);
    addr.parse().expect("Invalid host and/or port provided.")
}

fn with_data<T: Clone + Send>(data: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || data.clone())
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
        .and_then(handle_get_url);

    let post_url = warp::post()
        .and(warp::body::json())
        .and(with_data(db))
        .and_then(handle_post_url);

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
