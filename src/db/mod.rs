pub mod buckets;

use sqlx::SqlitePool;

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

#[cfg(test)]
mod test {
    use crate::db::init_pool;

    #[tokio::test]
    async fn test_init_pool() {
        let pool = init_pool("sqlite::memory:").await.unwrap();
        assert!(!pool.is_closed());
    }

    #[tokio::test]
    async fn test_migrations_applied() {
        let pool = init_pool("sqlite::memory:").await.unwrap();

        let result = sqlx::query("SELECT name FROM buckets LIMIT 1")
            .fetch_optional(&pool)
            .await;
        assert!(result.is_ok());
    }
}
