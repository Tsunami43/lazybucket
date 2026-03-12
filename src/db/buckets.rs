pub async fn create_bucket(pool: &sqlx::SqlitePool, name: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO buckets (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::db;

    #[tokio::test]
    async fn test_create_bucket() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();

        db::buckets::create_bucket(&pool, "test").await.unwrap();

        // Check duplicated insert
        let duplicated = db::buckets::create_bucket(&pool, "test").await;
        assert!(duplicated.is_err())
    }
}
