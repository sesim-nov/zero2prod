use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn insert_token_for_id(
    id: uuid::Uuid,
    pool: &sqlx::PgPool,
) -> Result<String, sqlx::Error> {
    let token = generate_token();
    sqlx::query!(
        r#"
        INSERT INTO tokens (subscriber_id, subscription_token)
        VALUES ($1, $2)
        "#,
        id,
        token
    )
    .execute(pool)
    .await?;
    Ok(token)
}

fn generate_token() -> String {
    let rng = thread_rng();
    rng.sample_iter(Alphanumeric)
        .map(char::from)
        .take(25)
        .collect()
}
