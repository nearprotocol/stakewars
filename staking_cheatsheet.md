# Staking Pool Cheatsheet

This low-maintained cheatsheet supports Validators and Delegators who use [near-shell](https://github.com/near/near-shell) to manage their stake.

Replace `nearkat.stakingpool` with the name of your pool, and `pool_admin.nearkat` with the owner/administrator of the pool (or the account that is staking the tokens).

If you get stuck, give a look to the [troubleshooting page](trobuleshooting.md), open an issue, or join [near.chat](https://near.chat) on Discord

## Deploy a staking pool using the staking pool factory
This is the only method allowed to deploy a staking pool on TestNet and MainNet, and uses the [staking pool factory](https://github.com/near/core-contracts/tree/master/staking-pool-factory). All pools deployed with this method on TestNet will end with the `.stakingpool` suffix.

### Call the staking pool factory `.stakingpool` to deploy the contract
```
near call stakingpool create_staking_pool '{"staking_pool_id":"nearkat", "owner_id":"pool_admin.nearkat", "stake_public_key":"ed25519:00000000000000000000000000000000000000000042", "reward_fee_fraction": {"numerator":25, "denominator": 100}}' --accountId pool_admin.nearkat --amount 30
```
From the example above, you have to replace:
- `nearkat` with the name of your staking pool (**HEADS UP:** the factory automatically adds its name to this parameter, creating `nearkat.stakingpool`)
- `pool_admin.nearkat` with the wallet that will control the staking pool
- `ed25519:0..042` with the public key in your `validator.json` file
- `25` with the fees that you like (in this case 25 over 100 is 25% of fees!)
- `pool_admin.nearkat` in the --accountId with your pool admin account
- be sure to have `30` NEAR available in your account (**HEADS UP:** keep the minimum balance to pay the [storage stake](https://near.org/papers/the-official-near-white-paper/#economics))

## Manage your deployed staking pool
Once your pool is deployed, you can issue the commands below

### Deposit 10k tokens for the account `pool_admin.nearkat`
```
near call nearkat.stakingpool deposit '' --accountId pool_admin.nearkat --amount 10000
```

### Stake 10k tokens (value in YoctoNEAR) with the account `pool_admin.nearkat`
```
near call nearkat.stakingpool stake '{"amount": "10000000000000000000000000000"}' --accountId pool_admin.nearkat
```

### Ping the pool, paying gas from account `pool_admin.nearkat`
```
near call nearkat.stakingpool ping '{}' --accountId pool_admin.nearkat
```

### Retrieve the total balance in YoctoNEAR for the account `pool_admin.nearkat`
```
near view nearkat.stakingpool get_account_total_balance '{"account_id": "pool_admin.nearkat"}'
```

### Retrieve the staked balance in YoctoNEAR for the account `pool_admin.nearkat`
```
near view nearkat.stakingpool get_account_staked_balance '{"account_id": "pool_admin.nearkat"}'
```

### Unstake 10k tokens (value in YoctoNEAR) from the account `pool_admin.nearkat`
```
near call nearkat.stakingpool unstake '{"amount": "10000000000000000000000000000"}' --accountId pool_admin.nearkat
```

### Retrieve the unstaked balance in YoctoNEAR for the account `pool_admin.nearkat`
```
near view nearkat.stakingpool get_account_unstaked_balance '{"account_id": "pool_admin.nearkat"}'
```

### Check if the unstaked balance for the account `pool_admin.nearkat` is unlocked
```
near view nearkat.stakingpool is_account_unstaked_balance_available '{"account_id": "pool_admin.nearkat"}'
```

### Withdraw 10k tokens in YoctoNEAR from the account `pool_admin.nearkat`
```
near call nearkat.stakingpool withdraw '{"amount": "10000000000000000000000000000"}' --accountId pool_admin.nearkat
```

### Retrieve the owner ID of the staking pool
```
near view nearkat.stakingpool get_owner_id '{}'
```

### Retrieve the staking key of the staking pool
```
near view nearkat.stakingpool get_staking_key '{}'
```

### Update the staking key of the staking pool
(replace the pubkey below with the key in your `validator.json` file)
```
near call nearkat.stakingpool update_staking_key '{"stake_public_key": "ed25519:00000000000000000000000000000000000000000042"}' --accountId pool_admin.nearkat
```

### Put the staking pool on hold
```
near call nearkat.stakingpool pause_staking '{}' --accountId pool_admin.nearkat
```

### Resume the staking
```
near call nearkat.stakingpool resume_staking '{}' --accountId pool_admin.nearkat
```


## Alternative Method: deploy a custom staking pool, using your locally-compiled contract
This method can be used only on BetaNet, and allows you to run your own fork of the [staking pool](https://github.com/near/core-contracts/tree/master/staking-pool).

### Deploy the smart contract in the account `my_cool_pool.nearkat`
(replace `my_cool_pool.nearkat` account with your cool pool name)
```
near deploy --accountId=my_cool_pool.nearkat --wasmFile=res/staking_pool_with_shares.wasm
```

### Configure the staking pool contract
(replace `my_cool_pool.nearkat`, `pool_admin.nearkat`, `stake_public_key` and `reward_fee_fraction` accordingly)
```
near call my_cool_pool.nearkat new '{"owner_id": "pool_admin.nearkat", "stake_public_key": "ed25519:00000000000000000000000000000000000000000042", "reward_fee_fraction": {"numerator": 25, "denominator": 100}}' --accountId pool_admin.nearkat
```
The pool above will have 25% of fees (25 numerator, 100 denominator).