use firebase_rs::RequestError;

use crate::{
    api_url::{StoryType, firebase, get_stories_url},
    cache::CacheItemType,
};

pub type StoriesResponse = Vec<usize>;

pub async fn get_stories(kind: StoryType) -> Result<StoriesResponse, RequestError> {
    let firebase = firebase();
    let url = get_stories_url(kind);
    let response = firebase.get(&url).await?;

    if let CacheItemType::Story(res) = response {
        Ok(res)
    } else {
        Err(RequestError::SerializeError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_stories() {
        let response = get_stories(StoryType::Top).await;
        assert!(response.is_ok());
        let stories = response.unwrap();
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|&story| story > 0));
    }
}
