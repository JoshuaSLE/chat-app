use anyhow::Context;
use backend::invite::{generate_code, hash_code};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await?;

    let code = generate_code();
    sqlx::query(
        "INSERT INTO invitations (id, code_hash, expires_at)
         VALUES ($1, $2, now() + interval '7 days')",
    )
    .bind(Uuid::now_v7())
    .bind(hash_code(&code))
    .execute(&pool)
    .await?;

    println!("Invite code (shown once): {code}");
    Ok(())
}
