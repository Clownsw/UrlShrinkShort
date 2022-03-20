use crate::pojo::url::{InsertUrl, SelectUrl};
use sqlx::mysql::{MySqlPool, MySqlQueryResult};

pub async fn select_by_name(
    db_pool: &MySqlPool,
    url_name: &String,
) -> Result<SelectUrl, sqlx::Error> {
    sqlx::query_as!(
        SelectUrl,
        r#"
            SELECT * FROM urls WHERE url_name = ?
        "#,
        url_name
    )
    .fetch_one(db_pool)
    .await
}

pub async fn select_by_target(
    db_pool: &MySqlPool,
    url_target: &String,
) -> Result<SelectUrl, sqlx::Error> {
    sqlx::query_as!(
        SelectUrl,
        r#"
            SELECT * FROM urls WHERE url_target = ?
        "#,
        url_target
    )
    .fetch_one(db_pool)
    .await
}

pub async fn insert_url(db_pool: &MySqlPool, url: &InsertUrl) -> MySqlQueryResult {
    sqlx::query!(
        r#"
            INSERT INTO urls(url_name, url_target, url_time)
            VALUES(?, ?, ?)
        "#,
        url.url_name,
        url.url_target,
        url.url_time
    )
    .execute(db_pool)
    .await
    .unwrap()
}
