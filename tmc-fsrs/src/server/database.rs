use sqlx::{postgres::PgPoolOptions, PgPool};

#[cfg(feature = "server")]
pub async fn connect_to_pgdb() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await?;
    Ok(pool)
}
