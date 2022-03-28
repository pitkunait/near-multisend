use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DocumentRequest {
  pub title: String,
  pub description: String,
  pub url: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Document {
  pub title: String,
  pub description: String,
  pub url: String,
  pub votes: u64,
  pub creator: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
  pub account: AccountId,
  pub available_votes: u64,
  pub voted: u64,
}
