use reqwest::Error;

use crate::consts::{get_stories_url, StoryType};

type StoriesResponse = Vec<usize>;

pub async fn get_showstories() -> Result<StoriesResponse, Error> {
    let url = get_stories_url(StoryType::Show);
    let response = reqwest::get(url).await?.json::<StoriesResponse>().await?;

    Ok(response)
}

pub async fn get_topstories() -> Result<StoriesResponse, Error> {
    let url = get_stories_url(StoryType::Top);
    let response = reqwest::get(url).await?.json::<StoriesResponse>().await?;

    Ok(response)
}

pub async fn get_newstories() -> Result<StoriesResponse, Error> {
    let url = get_stories_url(StoryType::New);
    let response = reqwest::get(url).await?.json::<StoriesResponse>().await?;

    Ok(response)
}

pub async fn get_beststories() -> Result<StoriesResponse, Error> {
    let url = get_stories_url(StoryType::Best);
    let response = reqwest::get(url).await?.json::<StoriesResponse>().await?;

    Ok(response)
}

pub async fn get_jobstories() -> Result<StoriesResponse, Error> {
    let url = get_stories_url(StoryType::Jobs);
    let response = reqwest::get(url).await?.json::<StoriesResponse>().await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_showstories() {
        let response = get_showstories().await;
        assert!(response.is_ok());
        let stories = response.unwrap();
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|&story| story > 0));
    }

    #[tokio::test]
    async fn test_get_topstories() {
        let response = get_topstories().await;
        assert!(response.is_ok());
        let stories = response.unwrap();
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|&story| story > 0));
    }

    #[tokio::test]
    async fn test_get_newstories() {
        let response = get_newstories().await;
        assert!(response.is_ok());
        let stories = response.unwrap();
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|&story| story > 0));
    }

    #[tokio::test]
    async fn test_get_beststories() {
        let response = get_beststories().await;
        assert!(response.is_ok());
        let stories = response.unwrap();
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|&story| story > 0));
    }

    #[tokio::test]
    async fn test_get_jobstories() {
        let response = get_jobstories().await;
        assert!(response.is_ok());
        let stories = response.unwrap();
        assert!(!stories.is_empty());
        assert!(stories.iter().all(|&story| story > 0));
    }
}
