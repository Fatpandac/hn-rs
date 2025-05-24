const API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0/";

const TOP_STORIES_ENDPOINT: &str = "topstories.json";
const NEW_STORIES_ENDPOINT: &str = "newstories.json";
const SHOW_STORIES_ENDPOINT: &str = "showstories.json";
const BEST_STORIES_ENDPOINT: &str = "beststories.json";
const JOBS_ENDPOINT: &str = "jobstories.json";

const ITEM_ENDPOINT: &str = "item/{}.json";

pub enum StoryType {
    Top,
    New,
    Show,
    Best,
    Jobs,
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
}

