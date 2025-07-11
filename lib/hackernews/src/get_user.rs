use firebase_rs::RequestError;
use serde::{Deserialize, Serialize};

use crate::api_url::{firebase, get_user_url};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub created: u64,
    pub karma: u64,
    pub about: Option<String>,
    pub submitted: Vec<u64>,
}

pub async fn get_user(user_name: &str) -> Result<User, RequestError> {
    let firebase = firebase();
    let url = get_user_url(user_name);
    let response = firebase.at(&url).get::<User>().await?;

    Ok(response)
}
