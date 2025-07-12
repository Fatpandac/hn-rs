use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::cache::FirebaseCache;

const API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0/";

const TOP_STORIES_ENDPOINT: &str = "topstories";
const NEW_STORIES_ENDPOINT: &str = "newstories";
const SHOW_STORIES_ENDPOINT: &str = "showstories";
const BEST_STORIES_ENDPOINT: &str = "beststories";
const JOBS_ENDPOINT: &str = "jobstories";

const ITEM_ENDPOINT: &str = "item/{}";
const USER_ENDPOINT: &str = "user/{}";

static FIREBASE: Lazy<Arc<FirebaseCache>> = Lazy::new(|| {
    Arc::new(FirebaseCache::new(
        API_BASE_URL,
        std::time::Duration::from_secs(60 * 5),
    ))
});

pub fn firebase() -> Arc<FirebaseCache> {
    FIREBASE.clone()
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StoryType {
    Top,
    New,
    Show,
    Best,
    Jobs,
}

impl ToString for StoryType {
    fn to_string(&self) -> String {
        match self {
            StoryType::Top => "Top Stories".to_string(),
            StoryType::New => "New Stories".to_string(),
            StoryType::Show => "Show Stories".to_string(),
            StoryType::Best => "Best Stories".to_string(),
            StoryType::Jobs => "Job Stories".to_string(),
        }
    }
}

pub fn get_stories_url(endpoint: StoryType) -> String {
    let endpoint = match endpoint {
        StoryType::Top => TOP_STORIES_ENDPOINT,
        StoryType::New => NEW_STORIES_ENDPOINT,
        StoryType::Show => SHOW_STORIES_ENDPOINT,
        StoryType::Best => BEST_STORIES_ENDPOINT,
        StoryType::Jobs => JOBS_ENDPOINT,
    };
    endpoint.to_string()
}

pub fn get_item_url(item_id: usize) -> String {
    ITEM_ENDPOINT.replace("{}", &item_id.to_string())
}

pub fn get_user_url(username: &str) -> String {
    USER_ENDPOINT.replace("{}", username)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_firebase() {
        let _ = firebase();
    }

    #[test]
    fn test_get_stories_url() {
        assert_eq!(get_stories_url(StoryType::Top), TOP_STORIES_ENDPOINT);
        assert_eq!(get_stories_url(StoryType::New), NEW_STORIES_ENDPOINT);
        assert_eq!(get_stories_url(StoryType::Show), SHOW_STORIES_ENDPOINT);
        assert_eq!(get_stories_url(StoryType::Best), BEST_STORIES_ENDPOINT);
        assert_eq!(get_stories_url(StoryType::Jobs), JOBS_ENDPOINT);
    }

    #[test]
    fn test_get_item_url() {
        let item_id = 12345;
        assert_eq!(get_item_url(item_id), format!("item/{}", item_id));
    }

    #[test]
    fn test_get_user_url() {
        let username = "testuser";
        assert_eq!(get_user_url(username), format!("user/{}", username));
    }
}
