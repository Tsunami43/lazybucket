use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
pub struct Object {
    pub bucket: String,
    pub key: String,
    pub size: i64,
    pub content_type: Option<String>,
    pub etag: String,
    pub storage_path: String,
    pub created_at: String,
}

pub async fn create_object(pool: &SqlitePool, obj: &Object) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO objects (bucket, key, size, content_type, etag, storage_path)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&obj.bucket)
    .bind(&obj.key)
    .bind(obj.size)
    .bind(&obj.content_type)
    .bind(&obj.etag)
    .bind(&obj.storage_path)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_object(
    pool: &SqlitePool,
    bucket: &str,
    key: &str,
) -> anyhow::Result<Option<Object>> {
    let row = sqlx::query_as::<_, Object>(
        "SELECT bucket, key, size, content_type, etag, storage_path, created_at
         FROM objects WHERE bucket = ? AND key = ?",
    )
    .bind(bucket)
    .bind(key)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn delete_object(pool: &SqlitePool, bucket: &str, key: &str) -> anyhow::Result<bool> {
    let result = sqlx::query("DELETE FROM objects WHERE bucket = ? AND key = ?")
        .bind(bucket)
        .bind(key)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn rename_object(
    pool: &SqlitePool,
    bucket: &str,
    key: &str,
    new_key: &str,
) -> anyhow::Result<bool> {
    let result = sqlx::query(
        "UPDATE objects SET key = ?, updated_at = CURRENT_TIMESTAMP WHERE bucket = ? AND key = ?",
    )
    .bind(new_key)
    .bind(bucket)
    .bind(key)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn list_objects(
    pool: &SqlitePool,
    bucket: &str,
    prefix: Option<&str>,
) -> anyhow::Result<Vec<Object>> {
    let rows = match prefix {
        Some(p) => {
            let pattern = format!("{}%", p);
            sqlx::query_as::<_, Object>(
                "SELECT bucket, key, size, content_type, etag, storage_path, created_at
                 FROM objects WHERE bucket = ? AND key LIKE ?
                 ORDER BY key",
            )
            .bind(bucket)
            .bind(pattern)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, Object>(
                "SELECT bucket, key, size, content_type, etag, storage_path, created_at
                 FROM objects WHERE bucket = ?
                 ORDER BY key",
            )
            .bind(bucket)
            .fetch_all(pool)
            .await?
        }
    };
    Ok(rows)
}

#[cfg(test)]
mod test {
    use super::Object;
    use crate::db;

    async fn setup() -> sqlx::SqlitePool {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        db::buckets::create_bucket(&pool, "test-bucket")
            .await
            .unwrap();
        pool
    }

    fn make_object(key: &str) -> Object {
        Object {
            bucket: "test-bucket".to_string(),
            key: key.to_string(),
            size: 42,
            content_type: Some("text/plain".to_string()),
            etag: "abc123".to_string(),
            storage_path: format!("test-bucket/{}", key),
            created_at: "2024-01-01 00:00:00".to_string(),
        }
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = setup().await;
        let obj = make_object("hello.txt");
        db::objects::create_object(&pool, &obj).await.unwrap();

        let found = db::objects::get_object(&pool, "test-bucket", "hello.txt")
            .await
            .unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().etag, "abc123");
    }

    #[tokio::test]
    async fn test_get_missing() {
        let pool = setup().await;
        let found = db::objects::get_object(&pool, "test-bucket", "nope.txt")
            .await
            .unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_delete_object() {
        let pool = setup().await;
        db::objects::create_object(&pool, &make_object("file.txt"))
            .await
            .unwrap();
        let deleted = db::objects::delete_object(&pool, "test-bucket", "file.txt")
            .await
            .unwrap();
        assert!(deleted);
        let not_found = db::objects::delete_object(&pool, "test-bucket", "file.txt")
            .await
            .unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    async fn test_rename_object() {
        let pool = setup().await;
        db::objects::create_object(&pool, &make_object("old.txt"))
            .await
            .unwrap();
        let renamed = db::objects::rename_object(&pool, "test-bucket", "old.txt", "new.txt")
            .await
            .unwrap();
        assert!(renamed);
        assert!(
            db::objects::get_object(&pool, "test-bucket", "new.txt")
                .await
                .unwrap()
                .is_some()
        );
        assert!(
            db::objects::get_object(&pool, "test-bucket", "old.txt")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_list_objects() {
        let pool = setup().await;
        db::objects::create_object(&pool, &make_object("cats/a.jpg"))
            .await
            .unwrap();
        db::objects::create_object(&pool, &make_object("cats/b.jpg"))
            .await
            .unwrap();
        db::objects::create_object(&pool, &make_object("dogs/c.jpg"))
            .await
            .unwrap();

        let all = db::objects::list_objects(&pool, "test-bucket", None)
            .await
            .unwrap();
        assert_eq!(all.len(), 3);

        let cats = db::objects::list_objects(&pool, "test-bucket", Some("cats/"))
            .await
            .unwrap();
        assert_eq!(cats.len(), 2);
    }
}
