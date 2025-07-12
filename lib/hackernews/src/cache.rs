use firebase_rs::{Firebase, RequestError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::get_items::ItemResponse;
use crate::get_stories::StoriesResponse;
use crate::get_user::User;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CacheItemType {
    Item(ItemResponse),
    Story(StoriesResponse),
    User(User),
}

#[derive(Clone)]
pub struct CacheData {
    value: CacheItemType,
    expire: Instant,
}

pub struct FirebaseCache {
    firebase: Firebase,
    cache: Arc<RwLock<HashMap<String, CacheData>>>,
    max_age: Duration,
}

impl FirebaseCache {
    pub fn new(url: &str, max_age: Duration) -> Self {
        FirebaseCache {
            firebase: Firebase::new(url).expect("Failed to create Firebase instance"),
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_age,
        }
    }

    pub async fn get(&self, key: &str) -> Result<CacheItemType, RequestError> {
        {
            let cache = self.cache.read().await;
            if let Some(data) = cache.get(key) {
                if data.expire > Instant::now() {
                    return Ok(data.value.clone());
                } else {
                    drop(cache);
                    self.remove(key).await;
                }
            }
        }

        let res = self.fetch_from_firebase(key).await?;
        self.set(key.to_string(), res.clone()).await;

        Ok(res)
    }

    async fn fetch_from_firebase(&self, key: &str) -> Result<CacheItemType, RequestError> {
        if key.starts_with("item/") {
            let item: ItemResponse = self.firebase.at(key).get().await?;
            Ok(CacheItemType::Item(item))
        } else if key.contains("stories") {
            let story: StoriesResponse = self.firebase.at(key).get().await?;
            Ok(CacheItemType::Story(story))
        } else if key.starts_with("user/") {
            let user: User = self.firebase.at(key).get().await?;
            Ok(CacheItemType::User(user))
        } else {
            Err(RequestError::SerializeError)
        }
    }

    pub async fn set(&self, key: String, value: CacheItemType) {
        let expire = Instant::now() + self.max_age;
        let mut cache = self.cache.write().await;
        cache.insert(key, CacheData { value, expire });
    }

    pub async fn remove(&self, key: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_firebase_cache() {
        let cache = FirebaseCache::new(
            "https://hacker-news.firebaseio.com/v0/",
            Duration::from_secs(5),
        );

        let user = User {
            id: "test_user".to_string(),
            created: 1234567890,
            karma: 100,
            about: Some("Test user".to_string()),
            submitted: vec![1, 2, 3],
        };
        cache
            .set("user/1".to_string(), CacheItemType::User(user.clone()))
            .await;
        let fetched_item = cache.get("user/1").await.unwrap();
        assert_eq!(fetched_item, CacheItemType::User(user.clone()));

        tokio::time::sleep(Duration::from_secs(6)).await;
        let expired_item = cache.get("user/1").await;
        assert!(expired_item.is_err());
    }
}
