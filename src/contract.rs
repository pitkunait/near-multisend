use crate::*;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
  Documents,
  Users,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  documents: UnorderedMap<u64, Document>,
  users: UnorderedMap<AccountId, User>,
  vote_cost: Balance,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      documents: UnorderedMap::new(StorageKey::Documents),
      users: UnorderedMap::new(StorageKey::Users),
      vote_cost: ONE_NEAR / 2,
    }
  }

  #[payable]
  pub fn add_document(&mut self, document: DocumentRequest) {
    // create new user if does not exist
    let account = env::predecessor_account_id();
    self.get_or_create_account(&account);

    // create new document
    let new_document = Document {
      title: document.title,
      description: document.description,
      url: document.url,
      votes: 0,
      creator: account,
    };
    let id = self.documents.len() + 1;
    self.documents.insert(&id, &new_document);
  }

  #[payable]
  pub fn place_vote(&mut self, id: U64) {
    let account = env::predecessor_account_id();
    let mut user = self.get_or_create_account(&account);
    assert!(user.available_votes > 0, "Not enough votes available!");

    match self.documents.get(&id.0) {
      Some(mut document) => {
        document.votes += 1;
        user.available_votes -= 1;
        self.documents.insert(&id.0, &document);
        self.users.insert(&account, &user);
      }
      None => env::panic_str("Document with given id is not found!"),
    }
  }

  #[payable]
  pub fn add_votes(&mut self) {
    let account = env::predecessor_account_id();
    let attached_deposit = env::attached_deposit();
    let mut user = self.get_or_create_account(&account);

    let votes = (attached_deposit / self.vote_cost) as u64;
    log!("votes: {}", votes);
    log!("attached deposit: {}", attached_deposit);
    log!("vote cost: {}", self.vote_cost);

    user.available_votes += votes;
    self.users.insert(&account, &user);

    self.refund(
      attached_deposit,
      votes as u128 * self.vote_cost,
      env::signer_account_id(),
    )
  }

  pub fn view_users(&self) -> Vec<(AccountId, User)> {
    self.users.to_vec()
  }

  pub fn view_documents(&self) -> Vec<(u64, Document)> {
    self.documents.to_vec()
  }

  fn get_or_create_account(&mut self, account: &AccountId) -> User {
    // checks if user exists, if not creates new account
    match self.users.get(account) {
      Some(user) => user,
      None => {
        let user = User {
          account: account.clone(),
          available_votes: 0,
          voted: 0,
        };
        self.users.insert(account, &user);
        user
      }
    }
  }

  #[payable]
  pub fn send_multiple(&mut self, accounts: HashMap<AccountId, U128>) {
    let attached_deposit = env::attached_deposit();

    let mut total_transferred: Balance = 0;
    for amount in accounts.values() {
      total_transferred += amount.0;
    }
    assert!(
      total_transferred < attached_deposit,
      "Not enough deposit to make transfer"
    );

    for (account, amount) in accounts {
      Promise::new(account).transfer(amount.0);
    }

    self.refund(
      attached_deposit,
      total_transferred,
      env::signer_account_id(),
    )
  }

  fn refund(&self, deposit: Balance, used_deposit: Balance, account: AccountId) {
    let refund = deposit as i128 - used_deposit as i128;
    if refund > 1 {
      Promise::new(account).transfer(refund as u128);
    }
  }
}
