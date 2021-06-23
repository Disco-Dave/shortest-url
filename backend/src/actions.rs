use std::convert::TryFrom;

use http::Uri;
use sqlx::PgPool;

use crate::base62::Base62;
use crate::database::urls;

pub enum GetUrlError {
    SlugIsEmpty,
    SlugContaintsInvalidChar(char),
    UrlNotFound,
    InvalidUrl(String),
    SqlError(sqlx::Error),
}

pub async fn get_url(slug: String, db: &PgPool) -> Result<Uri, GetUrlError> {
    use GetUrlError::*;

    let trimmed = slug.trim();

    if trimmed.is_empty() {
        Err(SlugIsEmpty)
    } else {
        let key = Base62::try_from(trimmed).map_err(SlugContaintsInvalidChar)?;

        let raw_url = urls::lookup_url(key.into(), db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => UrlNotFound,
                e => SqlError(e),
            })?;

        let uri = Uri::try_from(&raw_url).map_err(|_| InvalidUrl(raw_url))?;

        Ok(uri)
    }
}

pub enum PostUrlError {
    InvalidUrl,
    SqlError(sqlx::Error),
}

impl From<sqlx::Error> for PostUrlError {
    fn from(e: sqlx::Error) -> Self {
        Self::SqlError(e)
    }
}

pub async fn post_url(url: String, db: &PgPool) -> Result<Base62, PostUrlError> {
    use PostUrlError::*;

    let uri = Uri::try_from(&url).map_err(|_| InvalidUrl)?;
    let key = urls::insert_url(&uri, db).await?;

    Ok(key.into())
}
