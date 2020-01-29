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

    /// Returns given user's staked balance.
    pub fn get_user_stake(&mut self, account_id: AccountId) -> Balance {
        let user = self.users.get(&account_id).expect("User is missing");
        user.staked
    }
}

#[cfg(test)]
mod tests {
    use crate::StakingContract;
     use near_bindgen::{MockedBlockchain, Balance, BlockIndex};
    use near_bindgen::{PublicKey, VMContext, AccountId, testing_env};
    use near_bindgen::env::signer_account_id;

    fn staking() -> AccountId {
        "staking".to_string()
    }

    fn bob() -> AccountId {
        "bob".to_string()
    }

    struct VMContextBuilder {
        context: VMContext
    }

    impl VMContextBuilder {
        pub fn new() -> Self {
            Self {
                context: VMContext {
                    current_account_id: "".to_string(),
                    signer_account_id: "".to_string(),
                    signer_account_pk: vec![0, 1, 2],
                    predecessor_account_id: "".to_string(),
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
        }

        pub fn current_account_id(mut self, account_id: AccountId) -> Self {
            self.context.current_account_id = account_id;
            self
        }

        pub fn signer_account_id(mut self, account_id: AccountId) -> Self {
            self.context.signer_account_id = account_id;
            self
        }

        pub fn predecessor_account_id(mut self, account_id: AccountId) -> Self {
            self.context.predecessor_account_id = account_id;
            self
        }

        pub fn block_index(mut self, block_index: BlockIndex) -> Self {
            self.context.block_index = block_index;
            self
        }

        pub fn attached_deposit(mut self, amount: Balance) -> Self {
            self.context.attached_deposit = amount;
            self
        }

        pub fn finish(self) -> VMConext {
            self.context
        }
    }

    #[test]
    fn test_deposit_withdraw() {
        let mut contract = StakingContract::new("owner".to_string(), PublicKey::default());
        let deposit_amount  = 1_000_000;
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).attached_deposit(deposit_amount).finish());
        contract.deposit();
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).finish());
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
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).finish());
        contract.stake(deposit_amount);
        // 10 epochs later, unstake half of the money.
        testing_env!(VMContextBuilder::new().current_account_id(staking()).predecessor_account_id(bob()).block_index(EPOCH_LENGTH * 10).finish());
        assert_eq!(contract.get_user_stake(bob()), deposit_amount);
        contract.unstake(deposit_amount / 2);
        assert_eq!(contract.get_user_stake(bob()), deposit_amount / 2);
        assert_eq!(contract.get_user_balance(bob()), deposit_amount / 2);
    }
}
