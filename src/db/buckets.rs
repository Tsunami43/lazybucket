pub async fn create_bucket(pool: &sqlx::SqlitePool, name: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO buckets (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct Bucket {
    pub name: String,
    pub created_at: String,
}

pub async fn list_buckets(pool: &sqlx::SqlitePool) -> anyhow::Result<Vec<Bucket>> {
    let rows = sqlx::query_as::<_, Bucket>("SELECT name, created_at FROM buckets ORDER BY created_at")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn rename_bucket(
    pool: &sqlx::SqlitePool,
    name: &str,
    new_name: &str,
) -> anyhow::Result<bool> {
    let result = sqlx::query("UPDATE buckets SET name = ? WHERE name = ?")
        .bind(new_name)
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_bucket(pool: &sqlx::SqlitePool, name: &str) -> anyhow::Result<bool> {
    let result = sqlx::query("DELETE FROM buckets WHERE name = ?")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
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

    #[tokio::test]
    async fn test_list_buckets() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();

        db::buckets::create_bucket(&pool, "alpha").await.unwrap();
        db::buckets::create_bucket(&pool, "beta").await.unwrap();

        let list = db::buckets::list_buckets(&pool).await.unwrap();
        let names: Vec<_> = list.iter().map(|b| b.name.as_str()).collect();
        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[tokio::test]
    async fn test_rename_bucket() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();

        db::buckets::create_bucket(&pool, "old").await.unwrap();
        let renamed = db::buckets::rename_bucket(&pool, "old", "new")
            .await
            .unwrap();
        assert!(renamed);

        let list = db::buckets::list_buckets(&pool).await.unwrap();
        let names: Vec<_> = list.iter().map(|b| b.name.as_str()).collect();
        assert_eq!(names, vec!["new"]);

        // Несуществующий бакет
        let not_found = db::buckets::rename_bucket(&pool, "old", "other")
            .await
            .unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    async fn test_delete_bucket() {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();

        db::buckets::create_bucket(&pool, "test").await.unwrap();

        let deleted = db::buckets::delete_bucket(&pool, "test").await.unwrap();
        assert!(deleted);

        // Second delete — bucket already gone
        let not_found = db::buckets::delete_bucket(&pool, "test").await.unwrap();
        assert!(!not_found);
    }
}
