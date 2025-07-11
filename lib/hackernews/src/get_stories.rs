use firebase_rs::RequestError;

use crate::api_url::{firebase, get_stories_url, StoryType};

type StoriesResponse = Vec<usize>;

pub async fn get_stories(kind: StoryType) -> Result<StoriesResponse, RequestError> {
    let firebase = firebase();
    let url = get_stories_url(kind);
    let response = firebase.at(&url).get::<StoriesResponse>().await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_stories() {
        let response = get_stories(StoryType::Show).await;
        assert!(response.is_ok());
        let stories = response.unwrap();
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|&story| story > 0));
    }
}
