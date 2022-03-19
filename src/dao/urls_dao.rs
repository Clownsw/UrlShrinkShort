use crate::pojo::url::InsertUrl;
use sqlx::mysql::{MySqlPool, MySqlQueryResult};

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
