use firebase_rs::RequestError;
use serde::{Deserialize, Serialize};

use crate::{
    api_url::{firebase, get_user_url},
    cache::CacheItemType,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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
    let response = firebase.get(&url).await?;

    if let CacheItemType::User(user) = response {
        Ok(user)
    } else {
        Err(RequestError::SerializeError)
    }
}
