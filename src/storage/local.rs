use std::path::PathBuf;
use tokio::fs;
use tokio_util::io::ReaderStream;

pub struct LocalStorage {
    pub base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: &str) -> Self {
        LocalStorage {
            base_path: PathBuf::from(base_path),
        }
    }

    pub fn object_path(&self, bucket: &str, key: &str) -> PathBuf {
        self.base_path.join(bucket).join(key)
    }

    pub async fn write(&self, bucket: &str, key: &str, data: Vec<u8>) -> anyhow::Result<()> {
        let path = self.object_path(bucket, key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&path, data).await?;
        Ok(())
    }

    pub async fn read(&self, bucket: &str, key: &str) -> anyhow::Result<Vec<u8>> {
        let path = self.object_path(bucket, key);
        let data = fs::read(&path).await?;
        Ok(data)
    }

    pub async fn read_stream(&self, bucket: &str, key: &str) -> anyhow::Result<ReaderStream<fs::File>> {
        let path = self.object_path(bucket, key);
        let file = fs::File::open(&path).await?;
        Ok(ReaderStream::new(file))
    }

    pub async fn delete(&self, bucket: &str, key: &str) -> anyhow::Result<()> {
        let path = self.object_path(bucket, key);
        fs::remove_file(&path).await?;
        Ok(())
    }

    pub async fn rename(&self, bucket: &str, key: &str, new_key: &str) -> anyhow::Result<()> {
        let from = self.object_path(bucket, key);
        let to = self.object_path(bucket, new_key);
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::rename(&from, &to).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::LocalStorage;
    use tempfile::TempDir;

    fn temp_storage() -> (TempDir, LocalStorage) {
        let dir = TempDir::new().unwrap();
        let storage = LocalStorage::new(dir.path().to_str().unwrap());
        (dir, storage)
    }

    #[tokio::test]
    async fn test_write_and_read() {
        let (_dir, storage) = temp_storage();
        storage.write("bucket", "file.txt", b"hello".to_vec()).await.unwrap();
        let data = storage.read("bucket", "file.txt").await.unwrap();
        assert_eq!(data, b"hello");
    }

    #[tokio::test]
    async fn test_read_missing_file() {
        let (_dir, storage) = temp_storage();
        let result = storage.read("bucket", "missing.txt").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete() {
        let (_dir, storage) = temp_storage();
        storage.write("bucket", "file.txt", b"data".to_vec()).await.unwrap();
        storage.delete("bucket", "file.txt").await.unwrap();
        let result = storage.read("bucket", "file.txt").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rename() {
        let (_dir, storage) = temp_storage();
        storage.write("bucket", "old.txt", b"data".to_vec()).await.unwrap();
        storage.rename("bucket", "old.txt", "new.txt").await.unwrap();
        let data = storage.read("bucket", "new.txt").await.unwrap();
        assert_eq!(data, b"data");
        let old = storage.read("bucket", "old.txt").await;
        assert!(old.is_err());
    }

    #[tokio::test]
    async fn test_write_nested_key() {
        let (_dir, storage) = temp_storage();
        storage.write("bucket", "a/b/c.txt", b"nested".to_vec()).await.unwrap();
        let data = storage.read("bucket", "a/b/c.txt").await.unwrap();
        assert_eq!(data, b"nested");
    }
}
