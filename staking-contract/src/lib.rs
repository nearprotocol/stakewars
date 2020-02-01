use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{AccountId, Balance, BlockIndex, env, near_bindgen as near_bindgen_macro, Promise, PublicKey};
use near_bindgen::collections::Map as NearMap;

#[cfg(test)]
mod test_utils;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const EPOCH_LENGTH: BlockIndex = 60;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct User {
    pub account_id: AccountId,
    pub amount: Balance,
    pub stake: Balance,
    pub stake_height: BlockIndex,
}

impl User {
    pub fn new(account_id: &AccountId, amount: Balance) -> Self {
        Self {
            account_id: account_id.clone(),
            amount,
            stake: 0,
            stake_height: 0,
        }
    }

    pub fn stake(&mut self, amount: Balance) {
        self.amount -= amount;
        self.stake += amount;
        self.stake_height = env::block_index();
    }

    pub fn unstake(&mut self, amount: Balance) {
        self.stake -= amount;
        self.stake_height = env::block_index();
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
    pub prev_staked_amount: Balance,
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
            prev_staked_amount: 0,
            staked_amount: 0,
            users: NearMap::default(),
        }
    }

    /// Call to update state after epoch switched.
    pub fn ping(&mut self) {
        // Epoch passed, we received rewards and need to redistribute it to users.
        assert!(env::account_locked_balance() >= self.staked_amount, "The logic of the contract was broken");
        let reward = env::account_locked_balance() - self.staked_amount;
        println!("Reward: {:?}", reward);
        if reward > 0 {
            // (reward / staked_amount) * amount
            let mut new_users = vec![];
            for (account_id, mut user) in self.users.iter() {
                // TODO: replace with epoch id.
                if user.stake_height + EPOCH_LENGTH < env::block_index() {
                    user.stake = user.stake + (user.stake * reward) / self.staked_amount;
                    new_users.push((account_id, user));
                }
            }
            println!("New users: {:?}", new_users);
            for (account_id, user) in new_users {
                self.users.insert(&account_id, &user);
            }
        }
    }

    /// Call to deposit money.
    pub fn deposit(&mut self) {
        self.ping();
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
        self.ping();
        let account_id = env::predecessor_account_id();
        let mut user = self.users.get(&account_id).expect("User is missing");
        user.stake(amount);
        self.users.insert(&account_id, &user);
        println!("{:?} stake: {}, staked: {}, locked: {}", user, amount, self.staked_amount, env::account_locked_balance());
        self.staked_amount += amount;
        Promise::new(env::current_account_id()).stake(self.staked_amount, self.stake_public_key.clone());
    }

    /// Withdraws the non staked balance for given user.
    pub fn withdraw(&mut self, amount: Balance) {
        self.ping();
        let account_id = env::predecessor_account_id();
        let mut user = self.users.get(&account_id).expect("User is missing");
        user.withdraw(amount);
        self.users.insert(&account_id, &user);
        Promise::new(account_id).transfer(amount);
    }

    /// Request withdrawal for epoch + 2 by sending unstaking transaction for
    /// `current locked - (given user deposit + rewards)`
    pub fn unstake(&mut self, amount: Balance) {
        self.ping();
        let account_id = env::predecessor_account_id();
        let mut user = self.users.get(&account_id).expect("User is missing");
        assert!(self.staked_amount >= amount);
        user.unstake(amount);
        self.users.insert(&account_id, &user);
        self.staked_amount -= amount;
        println!("{:?} unstake {}, staked: {}, locked: {}", user, amount, self.staked_amount, env::account_locked_balance());
        Promise::new(env::current_account_id()).stake(self.staked_amount, self.stake_public_key.clone());
    }

    /// Returns given user's liquid balance.
    pub fn get_user_balance(&mut self, account_id: AccountId) -> Balance {
        let user = self.users.get(&account_id).expect("User is missing");
        user.amount
    }

    /// Returns given user's staked balance.
    pub fn get_user_stake(&mut self, account_id: AccountId) -> Balance {
        let user = self.users.get(&account_id).expect("User is missing");
        user.stake
    }
}

#[cfg(test)]
mod tests {
    use near_bindgen::{Balance, BlockIndex, MockedBlockchain};
    use near_bindgen::{AccountId, PublicKey, testing_env, VMContext};

    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_deposit_withdraw() {
        let mut contract = StakingContract::new("owner".to_string(), PublicKey::default());
        let deposit_amount  = 1_000_000;
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).attached_deposit(deposit_amount).finish());
        contract.deposit();
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).account_balance(deposit_amount).finish());
        assert_eq!(contract.get_user_balance(bob()), deposit_amount);
        contract.withdraw(deposit_amount);
        assert_eq!(contract.get_user_balance(bob()), 0);
    }

    #[test]
    fn test_stake_unstake() {
        let mut contract = StakingContract::new("owner".to_string(), PublicKey::default());
        let deposit_amount  = 1_000_000;
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).attached_deposit(deposit_amount).finish());
        contract.deposit();
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).account_balance(deposit_amount).finish());
        contract.stake(deposit_amount);
        // 10 epochs later, unstake half of the money.
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).block_index(EPOCH_LENGTH * 10).account_locked_balance(deposit_amount + 10).finish());
        assert_eq!(contract.get_user_stake(bob()), deposit_amount);
        contract.unstake(deposit_amount / 2);
        assert_eq!(contract.get_user_stake(bob()), deposit_amount / 2);
        assert_eq!(contract.get_user_balance(bob()), deposit_amount / 2);
    }
}
