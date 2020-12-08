use crate::result::Result;
use crate::POOL;
use chrono::Utc;
use uuid::Uuid;

pub async fn run(_body: serde_json::Value) -> Result<String> {
    let uuid = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc().to_string();
    let mut conn = POOL.begin().await.unwrap();

    sqlx::query!(
        r#"
        INSERT INTO org ( id, name, created_at, updated_at )
        VALUES ( ?1, ?2, ?3, ?4 )
        "#,
        uuid,
        "Dan",
        now,
        now
    )
    .execute(&mut conn)
    .await
    .unwrap();

    Ok(uuid)
}
