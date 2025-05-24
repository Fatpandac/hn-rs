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

