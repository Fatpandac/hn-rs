use reqwest::Error;
use serde::Deserialize;

use crate::consts::get_item_url;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Job,
    Story,
    Comment,
    Poll,
    PollOpt,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ItemResponse {
    id: usize,
    deleted: Option<bool>,
    r#type: ItemType,
    by: Option<String>,
    time: usize,
    text: Option<String>,
    dead: Option<bool>,
    parent: Option<usize>,
    poll: Option<usize>,
    kids: Option<Vec<usize>>,
    url: Option<String>,
    score: Option<usize>,
    title: String,
    parts: Option<Vec<usize>>,
    descendants: Option<usize>,
}

pub async fn get_item(item_id: usize) -> Result<ItemResponse, Error> {
    let url = get_item_url(item_id);
    let response = reqwest::get(url).await?.json::<ItemResponse>().await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_item() {
        let item_id = 8863; // Example item ID
        let response = get_item(item_id).await;
        println!("Response: {:?}", response);
        assert!(response.is_ok());
        let item = response.unwrap();
        assert_eq!(item.id, item_id);
        assert!(item.r#type == ItemType::Story || item.r#type == ItemType::Job);
    }
}
