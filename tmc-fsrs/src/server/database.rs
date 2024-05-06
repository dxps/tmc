use sqlx::{postgres::PgPoolOptions, PgPool};

#[cfg(feature = "server")]
pub async fn connect_to_pgdb() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect("postgres://tmc:tmc@localhost:5442/tmc")
        .await?;
    Ok(pool)
}
