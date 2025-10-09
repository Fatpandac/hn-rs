use firebase_rs::RequestError;
use serde::{Deserialize, Serialize};

use crate::{
    api_url::{firebase, get_item_url},
    cache::CacheItemType,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Job,
    Story,
    Comment,
    Poll,
    PollOpt,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ItemResponse {
    pub id: usize,
    deleted: Option<bool>,
    r#type: ItemType,
    pub by: Option<String>,
    pub time: usize,
    pub text: Option<String>,
    dead: Option<bool>,
    parent: Option<usize>,
    poll: Option<usize>,
    pub kids: Option<Vec<usize>>,
    pub children: Option<Vec<ItemResponse>>,
    pub url: Option<String>,
    score: Option<usize>,
    pub title: Option<String>,
    parts: Option<Vec<usize>>,
    descendants: Option<usize>,
}

impl Default for ItemResponse {
    fn default() -> Self {
        ItemResponse {
            id: 0,
            deleted: None,
            r#type: ItemType::Story,
            by: Some("Linux".to_string()),
            time: 0,
            text: Some("This is a default item".to_string()),
            dead: None,
            parent: None,
            poll: None,
            kids: None,
            children: None,
            url: None,
            score: None,
            title: None,
            parts: None,
            descendants: None,
        }
    }
}

pub async fn get_item(item_id: usize) -> Result<ItemResponse, RequestError> {
    let firebase = firebase();
    let item_url = get_item_url(item_id);
    let response = firebase.get(&item_url).await?;

    if let CacheItemType::Item(item) = response {
        Ok(item)
    } else {
        Err(RequestError::SerializeError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_item() {
        let item_id = 8863; // Example item ID
        let response = get_item(item_id).await;
        assert!(response.is_ok());
        let item = response.unwrap();
        assert_eq!(item.id, item_id);
        assert!(item.r#type == ItemType::Story || item.r#type == ItemType::Job);
    }
}
