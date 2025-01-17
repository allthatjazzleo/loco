//! # Single Storage Strategy Implementation
//!
//! This module provides an implementation of the [`StorageStrategyTrait`] for a
//! single storage strategy.
use std::path::Path;

use bytes::Bytes;

use crate::storage::{
    error::{StorageError, StorageResult, StoreError},
    strategies::StorageStrategyTrait,
    Storage,
};

/// Represents a single storage strategy.
#[derive(Clone)]
pub struct SingleStrategy {
    pub primary: String,
}

impl SingleStrategy {
    /// Creates a new instance of `SingleStrategy` with the specified primary
    /// storage identifier.
    #[must_use]
    pub fn new(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
        }
    }
}

/// Implementation of `StorageStrategyTrait` for a single storage strategy.
#[async_trait::async_trait]
impl StorageStrategyTrait for SingleStrategy {
    /// Uploads content to the primary storage.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageResult`] indicating of the operation status.
    async fn upload(&self, storage: &Storage, path: &Path, content: &Bytes) -> StorageResult<()> {
        storage
            .as_store_err(&self.primary)?
            .upload(path, content)
            .await?;
        Ok(())
    }

    /// Downloads content
    ///
    /// # Errors
    ///
    /// Returns a [`StorageResult`] indicating of the operation status.
    async fn download(&self, storage: &Storage, path: &Path) -> StorageResult<Bytes> {
        let store = storage.as_store_err(&self.primary)?;
        Ok(store
            .get(path)
            .await?
            .bytes()
            .await
            .map_err(|e| StorageError::Storage(StoreError::Storage(e)))?)
    }

    /// Deletes the given path
    ///
    /// # Errors
    ///
    /// Returns a [`StorageResult`] indicating of the operation status.
    async fn delete(&self, storage: &Storage, path: &Path) -> StorageResult<()> {
        Ok(storage.as_store_err(&self.primary)?.delete(path).await?)
    }

    /// Renames the file name
    ///
    /// # Errors
    ///
    /// Returns a [`StorageResult`] indicating of the operation status.
    async fn rename(&self, storage: &Storage, from: &Path, to: &Path) -> StorageResult<()> {
        Ok(storage
            .as_store_err(&self.primary)?
            .rename(from, to)
            .await?)
    }

    /// Copy file from the given path to the new path
    ///
    /// # Errors
    ///
    /// Returns a [`StorageResult`] indicating of the operation status.
    async fn copy(&self, storage: &Storage, from: &Path, to: &Path) -> StorageResult<()> {
        Ok(storage.as_store_err(&self.primary)?.copy(from, to).await?)
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::BTreeMap, path::PathBuf};

    use super::*;
    use crate::storage::{drivers, Storage};

    #[tokio::test]
    async fn can_upload() {
        let store = drivers::mem::new();

        let strategy = Box::new(SingleStrategy::new("default")) as Box<dyn StorageStrategyTrait>;

        let storage = Storage::new(
            BTreeMap::from([("default".to_string(), store.clone())]),
            strategy.into(),
        );

        let path = PathBuf::from("users").join("data").join("1.txt");
        let file_content = Bytes::from("file content");

        assert!(storage.upload(path.as_path(), &file_content).await.is_ok());

        assert!(store.exists(path.as_path()).await.unwrap());
    }

    #[tokio::test]
    async fn can_download() {
        let store = drivers::mem::new();

        let strategy = Box::new(SingleStrategy::new("default")) as Box<dyn StorageStrategyTrait>;

        let storage = Storage::new(
            BTreeMap::from([("default".to_string(), store.clone())]),
            strategy.into(),
        );

        let path = PathBuf::from("users").join("data").join("1.txt");
        let file_content = Bytes::from("file content");

        assert!(store.upload(path.as_path(), &file_content).await.is_ok());

        let download_file: String = storage.download(path.as_path()).await.unwrap();
        assert_eq!(download_file, file_content);
    }

    #[tokio::test]
    async fn can_delete() {
        let store = drivers::mem::new();

        let strategy = Box::new(SingleStrategy::new("default")) as Box<dyn StorageStrategyTrait>;

        let storage = Storage::new(
            BTreeMap::from([("default".to_string(), store.clone())]),
            strategy.into(),
        );

        let path = PathBuf::from("users").join("data").join("1.txt");
        let file_content = Bytes::from("file content");

        assert!(store.upload(path.as_path(), &file_content).await.is_ok());

        assert!(store.exists(path.as_path()).await.unwrap());

        assert!(storage.delete(path.as_path()).await.is_ok());

        assert!(!store.exists(path.as_path()).await.unwrap());
    }

    #[tokio::test]
    async fn can_rename_file_path() {
        let store = drivers::mem::new();

        let strategy = Box::new(SingleStrategy::new("default")) as Box<dyn StorageStrategyTrait>;

        let storage = Storage::new(
            BTreeMap::from([("default".to_string(), store.clone())]),
            strategy.into(),
        );

        let orig_path = PathBuf::from("users").join("data").join("1.txt");
        let file_content = Bytes::from("file content");

        assert!(storage
            .upload(orig_path.as_path(), &file_content)
            .await
            .is_ok());

        assert!(store.exists(orig_path.as_path()).await.unwrap());

        let new_path = PathBuf::from("users").join("data-2").join("2.txt");
        assert!(storage
            .rename(orig_path.as_path(), new_path.as_path())
            .await
            .is_ok());

        assert!(!store.exists(orig_path.as_path()).await.unwrap());
        assert!(store.exists(new_path.as_path()).await.unwrap());
    }

    #[tokio::test]
    async fn can_copy_file_path() {
        let store = drivers::mem::new();

        let strategy = Box::new(SingleStrategy::new("default")) as Box<dyn StorageStrategyTrait>;

        let storage = Storage::new(
            BTreeMap::from([("default".to_string(), store.clone())]),
            strategy.into(),
        );

        let orig_path = PathBuf::from("users").join("data").join("1.txt");
        let file_content = Bytes::from("file content");

        assert!(storage
            .upload(orig_path.as_path(), &file_content)
            .await
            .is_ok());

        assert!(store.exists(orig_path.as_path()).await.unwrap());

        let new_path = PathBuf::from("users").join("data-2").join("2.txt");
        assert!(storage
            .copy(orig_path.as_path(), new_path.as_path())
            .await
            .is_ok());

        assert!(store.exists(orig_path.as_path()).await.unwrap());
        assert!(store.exists(new_path.as_path()).await.unwrap());
    }
}
