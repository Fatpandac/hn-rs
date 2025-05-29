const API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0/";

const TOP_STORIES_ENDPOINT: &str = "topstories.json";
const NEW_STORIES_ENDPOINT: &str = "newstories.json";
const SHOW_STORIES_ENDPOINT: &str = "showstories.json";
const BEST_STORIES_ENDPOINT: &str = "beststories.json";
const JOBS_ENDPOINT: &str = "jobstories.json";

const ITEM_ENDPOINT: &str = "item/{}.json";
const USER_ENDPOINT: &str = "user/{}.json";

#[derive(Clone, Copy, PartialEq)]
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
    format!("{}{}", API_BASE_URL, endpoint)
}

pub fn get_item_url(item_id: usize) -> String {
    format!("{}{}", API_BASE_URL, ITEM_ENDPOINT.replace("{}", &item_id.to_string()))
}

pub fn get_user_url(username: &str) -> String {
    format!("{}{}", API_BASE_URL, USER_ENDPOINT.replace("{}", username))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stories_url() {
        assert_eq!(get_stories_url(StoryType::Top), format!("{}{}", API_BASE_URL, TOP_STORIES_ENDPOINT));
        assert_eq!(get_stories_url(StoryType::New), format!("{}{}", API_BASE_URL, NEW_STORIES_ENDPOINT));
        assert_eq!(get_stories_url(StoryType::Show), format!("{}{}", API_BASE_URL, SHOW_STORIES_ENDPOINT));
        assert_eq!(get_stories_url(StoryType::Best), format!("{}{}", API_BASE_URL, BEST_STORIES_ENDPOINT));
        assert_eq!(get_stories_url(StoryType::Jobs), format!("{}{}", API_BASE_URL, JOBS_ENDPOINT));
    }

    #[test]
    fn test_get_item_url() {
        let item_id = 12345;
        assert_eq!(get_item_url(item_id), format!("{}item/{}.json", API_BASE_URL, item_id));
    }

    #[test]
    fn test_get_user_url() {
        let username = "testuser";
        assert_eq!(get_user_url(username), format!("{}user/{}.json", API_BASE_URL, username));
    }
}

