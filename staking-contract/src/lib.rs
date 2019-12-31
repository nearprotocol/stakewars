use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{env, near_bindgen as near_bindgen_macro, BlockIndex, AccountId, Balance, Promise, PublicKey};
use near_bindgen::collections::Map as NearMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const EPOCH_LENGTH: BlockIndex = 60;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct User {
    pub account_id: AccountId,
    pub amount: Balance,
    pub staked: Balance,
    pub staked_height: BlockIndex,
    pub locked: Balance,
    pub locked_height: BlockIndex,
}

impl User {
    pub fn new(account_id: &AccountId, amount: Balance) -> Self {
        Self {
            account_id: account_id.clone(),
            amount,
            staked: 0,
            staked_height: 0,
            locked: 0,
            locked_height: 0
        }
    }

    pub fn stake(&mut self, amount: Balance) {
        self.staked += amount;
        self.staked_height = env::block_index();
    }

    pub fn unstake(&mut self, amount: Balance) {
        self.locked += amount;
        self.locked_height = env::block_index();
    }

    /// Checks if given user has enough non staked/locked balance and withdraws it.
    pub fn withdraw(&mut self, amount: Balance) {
        // TODO
        self.amount -= amount;
    }
}

#[near_bindgen_macro]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakingContract {
    pub owner: AccountId,
    pub stake_public_key: PublicKey,
    pub staked_amount: Balance,
    pub users: NearMap<AccountId, User>,
}

#[near_bindgen_macro(init => new)]
impl StakingContract {
    /// Call to initialize the contract.
    /// Specify which account can change the staking key and the initial staking key.
    pub fn new(owner: AccountId, stake_public_key: PublicKey) -> Self {
        Self {
            owner,
            stake_public_key,
            staked_amount: 0,
            users: NearMap::default(),
        }
    }

    /// Call to deposit money.
    pub fn deposit(&mut self) {
        let account_id = env::predecessor_account_id();
        let user = if let Some(mut user) = self.users.get(&account_id) {
            user.amount += env::attached_deposit();
            user
        } else {
            User::new(&account_id, env::attached_deposit())
        };
        self.users.insert(&account_id, &user);
    }

    /// Stakes previously deposited money by given user on this account.
    pub fn stake(&mut self, amount: Balance) {
        let account_id = env::predecessor_account_id();
        let mut user = self.users.get(&account_id).expect("User is missing");
        user.stake(amount);
        self.users.insert(&account_id, &user);
        self.staked_amount += amount;
        Promise::new(env::current_account_id()).stake(self.staked_amount, self.stake_public_key.clone());
    }

    /// Withdraws the non staked balance for given user.
    pub fn withdraw(&mut self, amount: Balance) {
        let account_id = env::predecessor_account_id();
        let mut user = self.users.get(&account_id).expect("User is missing");
        user.withdraw(amount);
        self.users.insert(&account_id, &user);
        Promise::new(account_id).transfer(amount);
    }

    /// Request withdrawal for epoch + 2 by sending unstaking transaction for
    /// `current locked - (given user deposit + rewards)`
    pub fn unstake(&mut self, amount: Balance) {
        let account_id = env::predecessor_account_id();
        let mut user = self.users.get(&account_id).expect("User is missing");
        assert!(self.staked_amount >= amount);
        user.unstake(amount);
        self.users.insert(&account_id, &user);
        self.staked_amount -= amount;
        Promise::new(env::current_account_id()).stake(self.staked_amount, self.stake_public_key.clone());
    }

    /// Returns given user's liquid balance.
    pub fn get_user_balance(&mut self, account_id: AccountId) -> Balance {
        let user = self.users.get(&account_id).expect("User is missing");
        user.amount
    }
}

#[cfg(test)]
mod tests {
    use crate::StakingContract;
     use near_bindgen::{MockedBlockchain, Balance};
    use near_bindgen::{PublicKey, VMContext, AccountId, testing_env};

    fn staking() -> AccountId {
        "staking".to_string()
    }

    fn bob() -> AccountId {
        "bob".to_string()
    }

    fn get_context(predecessor_account_id: AccountId, attached_deposit: Balance) -> VMContext {
        VMContext {
            current_account_id: staking(),
            signer_account_id: "test".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit,
            prepaid_gas: 10u64.pow(9),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn test_deposit_withdraw() {
        let mut contract = StakingContract::new("owner".to_string(), PublicKey::default());
        let deposit_amount  = 1_000_000;
        testing_env!(get_context(bob(), deposit_amount));
        contract.deposit();
        assert_eq!(contract.get_user_balance(bob()), deposit_amount);
        contract.withdraw(deposit_amount);
        assert_eq!(contract.get_user_balance(bob()), 0);
    }
}
