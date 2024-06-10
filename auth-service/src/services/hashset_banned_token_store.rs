use std::collections::HashSet;
use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
  tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
  async fn insert(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
    if self.tokens.contains(token) {
      return Err(BannedTokenStoreError::AlreadyBanned);
    }
    self.tokens.insert(token.to_owned());
    Ok(())
  }

  async fn is_banned(
    &self,
    token: &str,
  ) -> bool {
    return self.tokens.contains(token);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_add_ban() {
    let mut banned_store = HashsetBannedTokenStore::default();
    let result = banned_store.insert("ban this").await;
    assert!(result.is_ok());
    let result = banned_store.insert("ban this").await;
    assert!(result.is_err());
  }

  #[tokio::test]
  async fn test_is_banned() {
    let mut banned_store = HashsetBannedTokenStore::default();
    let result = banned_store.insert("ban this").await;
    assert!(result.is_ok());
    assert!(banned_store.is_banned("ban this").await);
  }

  #[tokio::test]
  async fn test_not_banned() {
    let mut banned_store = HashsetBannedTokenStore::default();
    let result = banned_store.insert("ban this").await;
    assert!(result.is_ok());
    assert!(!banned_store.is_banned("not banned").await);
  }
}
