use http::Uri;
use sqlx::PgPool;

pub async fn lookup_url(key: u64, db: &PgPool) -> Result<String, sqlx::Error> {
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

pub async fn insert_url(uri: &Uri, db: &PgPool) -> Result<u64, sqlx::Error> {
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
