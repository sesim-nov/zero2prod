use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Insert a randomly generated token for the given subscriber ID, then return the token to the
/// caller.
pub async fn insert_token_for_id(
    id: uuid::Uuid,
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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

/// Query the token table for a token matching the provided ID. Return token to caller.
pub async fn get_token_for_id(
    id: uuid::Uuid,
    pool: &sqlx::PgPool,
) -> Result<Option<String>, sqlx::Error> {
    let query_result = sqlx::query!(
        r#"
        SELECT subscription_token FROM tokens
        WHERE subscriber_id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(query_result.map(|a| a.subscription_token))
}

/// Query the token table for an ID matching the provided token. Return ID to caller.
pub async fn get_id_for_token(
    token: String,
    pool: &sqlx::PgPool,
) -> Result<Option<uuid::Uuid>, sqlx::Error> {
    let query_result = sqlx::query!(
        r#"
        SELECT subscriber_id FROM tokens
        WHERE subscription_token = $1
        "#,
        token
    )
    .fetch_optional(pool)
    .await?;
    Ok(query_result.map(|a| a.subscriber_id))
}

/// Randomly generate a subscription token.
fn generate_token() -> String {
    let rng = thread_rng();
    rng.sample_iter(Alphanumeric)
        .map(char::from)
        .take(25)
        .collect()
}
