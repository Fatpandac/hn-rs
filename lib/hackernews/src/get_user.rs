use reqwest::Error;
use serde::Deserialize;

use crate::api_url::get_user_url;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub created: u64,
    pub karma: u64,
    pub about: Option<String>,
    pub submitted: Vec<u64>,
}

pub async fn get_user(user_name: &str) -> Result<User, Error> {
    let url = get_user_url(user_name);
    let response = reqwest::get(url).await?.json::<User>().await?;

    Ok(response)
}
