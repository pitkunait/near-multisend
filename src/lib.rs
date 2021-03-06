use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::UnorderedMap,
  env,
  json_types::{U128, U64},
  log, near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise, ONE_NEAR,
};
use std::collections::HashMap;

mod contract;
mod models;

pub use contract::*;
pub use models::*;

#[cfg(test)]
mod tests {
  use crate::*;
  use near_sdk::{test_utils::*, testing_env};

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn contract_account() -> AccountId {
    "contract".parse::<AccountId>().unwrap()
  }

  fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(contract_account())
      .account_balance(15 * ONE_NEAR)
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder
  }

  #[test]
  fn test() {}
}
