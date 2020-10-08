# Stake Wars Challenge 001
Published on: May 25th 2020
Updated on: July 22nd 2020

Create your BetaNet wallet, deploy your node, and correctly configure your staking pool.
This is designed to be your very first challenge: use it to understand how the staking on NEAR works, and how to run a node.


## Acceptance Criteria

1. Connect `near-cli` to your BetaNet wallet
2. Deploy your NEAR node
3. Launch your Staking Pool

## 1.Connect `near-cli` to your BetaNet wallet

You need [near-cli](https://github.com/near/near-cli), a Node.js application that relies on nearlib to generate secure keys, connect to the NEAR platform and send transactions to the network on your behalf.
You don't have to install near-cli on your validator node - on the contrary, it's suggested to keep your staking wallet on a different machine.

**Heads up:** please be sure to have Node version 10+ installed before trying to run near shell

1. Download `near-cli` with `npm install -g near-cli`
2. Set your NODE_ENV to betanet. You can do this in `bash` with the command `export NODE_ENV=betanet`
3. Run `near login` and copy/paste in your browser the link from the instructions on the screen
4. If you are not automatically redirected, manually type the `account_ID` of your BetaNet account in near-cli

**Heads up:** Even if you see an `ERR_CONNECTION_REFUSED` error in your browser, your login may be successful. So complete the step 5 and manually type your `account_ID` in near-cli.

This process is needed to authorize a `signer key` on your `account_ID`, enabling near-cli to control your wallet.

## 2.Deploy your NEAR node

Setup your validator node following the hardware requirements [here](https://docs.near.org/docs/roles/validator/hardware). If you use a firewall, set the rules to allow traffic to port `24567` from all IPs (0.0.0.0/0).
Once your machine is ready, install [nearup](https://github.com/near/nearup). Nearup will provide simplified tools to run NEAR nodes, and is designed to help validators start their nodes, and developers who need a local RPC for their applications.

Take your time to understand how to manually update [nearcore](https://github.com/nearprotocol/nearcore), and how to properly use the command `nearup betanet --nodocker --binary-path` (see below for contribution opportunities).
To connect your node to `betanet` download and compile the latest [nearcore release](https://github.com/nearprotocol/nearcore/releases):
- BetaNet releases of nearcore are mapped with the tag `x.y.z-beta`
- TestNet releases of nearcore are mapped with the tag `x.y.z-rc`

**Heads up:** at this point, you have to decide the name of your staking pool!
If your wallet is `nearkat.betanet`, you will have to choose a specific name for the staking pool, such as `nearkat_staking`. Since in the [step 3.2](challenge001.md#32-deploy-your-staking-pool) you will use the staking pool factory, your name will become `stakingPool_ID` + `stakehouse.betanet`.

The first time you run `nearup` the application will ask your staking pool account ID, which is where you have to type `nearkat_staking.stakehouse.betanet` if the name of the pool is just `nearkat_staking`:
```
Enter your account ID (leave empty if not going to be a validator):
```
As soon as you press enter nearup with automatically boot your validator node with the name you decided above. 

When nearup completed the boot process, export your validator key by typing the command `cat ~/.near/betanet/validator_key.json | grep public_key`, as it will be needed below to configure the staking pool contract.

## 3.Launch your Staking Pool

Once your validator node is running under the name you decided, you have to connect the staking pool to it. The steps below will guide you in this process:

### 3.1. Preparation: unstake any previously locked funds

If you were already a validator on BetaNet, and you are using the legacy `near stake` command, follow the commands below before proceeding:
1. Issue `export NODE_ENV=betanet` command before performing any unstaking, so near-cli will connect to `betanet` RPC.
2. Unstake your previously locked funds, by setting stake to zero: `near stake account_ID <staking public key> 0` - where the public key and the account ID are the same you used when you initially issued the staking transaction.

Please note that your staked funds will require three epochs to be available (9 hours on BetaNet). Once the funds are unlocked, you can stop nearup with the command `nearup stop`, cleanup the folder `~/.near/betanet` on your validator node, and start nearcore again, following the process from the step 2 above.

If you are running an old version of the staking pool jump to [step 3.4](challenge001.md#34-update-an-old-version-of-the-staking-pool-optional) before proceeding.

Once your node is not anymore a validator using the legacy `near stake` command you can begin deploy the staking pool.

### 3.2. Deploy your staking pool

Updated July 22nd: to accommodate MainNet Restricted requirements, we introduced a [Staking Pool Factory](https://github.com/near/core-contracts/tree/master/staking-pool-factory). This smart contract will deploy the staking pool for you, and will _whitelist_ the contract to receive and send tokens to your wallet. If you have any previous release of the staking pool, we strongly suggest to deploy this new version and dismiss the previous staking pool. 

Updated August 21st: You can use the staking pool factory via this web interface [Near Examples: Staking Pool Factory (betanet)](https://near-examples.github.io/staking-pool-factory) or you can invoke the factory's `create_staking_pool` function from the command line using `near-cli`

The staking pool factory is a normal smart contract that can be invoked with the `call` function on near-cli:
```
near call stakehouse.betanet create_staking_pool '{"staking_pool_id":"<POOL_ID>", "owner_id":"<OWNER_ID>", "stake_public_key":"<VALIDATOR_KEY>", "reward_fee_fraction": {"numerator": <X>, "denominator": <Y>}}' --account_id <OWNER_ID> --amount 30 --gas 300000000000000
```
Where:
* `stakehouse.betanet` is the staking pool factory contract
* `POOL_ID` is the name of the staking pool contract. If your validator name is `nearkat` the result will be `nearkat.pool.6fb1358`
* `OWNER_ID` is the owner of the pool, who's authorized to change the stake public key and the fees
* `VALIDATOR_KEY` is the public key found in the file `~/.near/testnet/validator_key.json` on the machine running the node
* `{"numerator": <X>, "denominator": <Y>}` set the validator fees. To set 10% of fees x=10 and y=100
* `--amount 30` attaches 30 $NEAR to the transaction, as a reserve to pay the contract storage
* `--gas 300000000000000` specifies the gas for the transaction (optional!)

### 3.3. Delegate your own unstaked funds to the staking pool

1. From your near-cli machine, deposit the funds from your master account to the staking pool: `near call stakingPool_ID deposit '{}' --accountId account_ID --amount 100`
	Where 100 is the amount in NEAR tokens you want to deposit.
2. Stake your deposited funds, with the command `near call stakingPool_ID stake '{"amount": "100000000000000000000000000"}' --accountId account_ID`

**Heads up:** the amount that you deposit is in $NEAR, while the amount in the stake argument is in YoctoNEAR. `1` $NEAR is `1*10^24` YoctoNEAR (1 followed by 24 zeroes). Therefore:

| NEAR |  YoctoNEAR  | YoctoNEAR |
| ---- | ----------- | ----------------|
| `1` | `1*10^24` | `1000000000000000000000000` |
| `10` | `1*10^25` | `10000000000000000000000000` |
| `100` | `1*10^26` | `100000000000000000000000000` |
| `1,000` | `1*10^27` | `1000000000000000000000000000` |
| `10,000` | `1*10^28` | `10000000000000000000000000000` |

We strongly suggest to get confidence with the `staking pool` contract, by reading more on the official initial-contracts repo on [Github](https://github.com/near/core-contracts/tree/master/staking-pool). Pay particular attention to the distinction between `stakingPool_ID` and `account_ID`.

### 3.4. Update an old version of the staking pool (optional)

The failsafe method is to deploy a new `stakingPool_ID` from step 3.2 of this challenge, unstake your funds from the old pool with the command `near call stakingPool_ID unstake`, and use them on the new pool. Only when the new node becomes validator, you can shut down the old node, and remove the pool.

At a high level, the steps to follow are:

1. View the total funds locked in the pool: `near view stakingPool_ID get_account_staked_balance '{"account_id": "account_ID"}'`
1. Unstake all your funds from the pool: `near call stakingPool_ID unstake '{"amount": "100007548461634906498130995"}' --accountId account_ID`
2. **IMPORTANT** Wait for 3 epochs (9 hours on BetaNet) to withdraw. Check if the stake is unlocked with the command `near view stakingPool_ID is_account_unstaked_balance_available '{"account_id": "account_ID"}' --accountId account_ID`
3. Once your funds are unlocked, withdraw them with the command `near call stakingPool_ID withdraw '{"amount": "100007548461634906498130995"}' --accountId account_ID`

Once completed, you are ready to get back to the step 3.2 and deploy a new staking pool contract. You may use the [Staking Pool Cheatsheet](../staking_cheatsheet.md) as a quick reference to use your staking pool.

## Contribution Opportunities

Do you want to earn extra tokens? We have contribution opportunities available below! 

Reply to the challenges application thread on [Github](https://github.com/nearprotocol/stakewars/issues/350) specifying:
- which contribution you want to do, and the link to the challenge
- the type of document you will release
- the date when you will publish the content (you can postpone a few days, no worries)

Once your work is done, you will be added to the list below. Please note that rewards in tokens will need to pass basic KYC checks from NEAR Foundation, and comply with regulations.

### List of contributions
**Heads Up:** No more contributions are accepted, except the ones already posted on https://portal.near.org before October 2nd. You are free to add new ones without receiving token rewards!

| Abstract | Description                    | Contributor | Publication | Link | NEAR Tokens |
| -------- | ------------------------------ | ----------- | ----------- | ---- | ----------- |
| How to compile nearcore | Create a tutorial, in the form of a video, a blogpost or Github document, that explains how to clone, compile and run `nearcore` from the beta branch, and prove that you can become validator with the process. Contributions in other languages are encouraged, but considered case by case basis. Updates to this guide, reflecting any updates of the tools involved, will be paid a % of the initial bounty per each revision, up to once per month, until Stake Wars is over. | @htafolla | Jun 8 2020 | PRs at [1](https://github.com/nearprotocol/stakewars#quickstart), [2](https://docs.near.org/docs/validator/staking), [3](https://docs.near.org/docs/local-setup/running-testnet) | 1,000 |
| How to compile nearcore | Same as above | @navneetlakra | Jun 8 2020 | [GitHub](https://github.com/mutedtommy/near-docs/blob/master/compile-nearcore.md) | 1,000 |
| How to compile nearcore | Same as above | @georgec138 | Jun 8 2020 | [GitHub](https://github.com/georgec138/near-docs/blob/master/COMPILE-NEARCORE_cn.md) | 1,000 |
| How to compile nearcore | Same as above | @narniec | Jun 8 2020 | [Medium](https://medium.com/@narniec2020/%D0%B7%D0%B0%D0%BF%D1%83%D1%81%D0%BA-%D0%BD%D0%BE%D0%B4%D1%8B-near-%D0%B2-%D1%82%D0%B5%D1%81%D1%82%D0%BE%D0%B2%D0%BE%D0%B9-%D1%81%D0%B5%D1%82%D0%B8-betanet-a4d58c9a7bb0) | 1,000 |
| How to compile nearcore | Same as above | @wjdfx | Jun 15 2020 | [Jianshu](https://www.jianshu.com/p/c962edcb1099) | 1,000 |
| How to compile nearcore | Same as above | @dreamstaker | Jun 23 2020 | [Medium](https://medium.com/@nodesanna/stake-wars-challenge-001-d87d207db948) | 1,000 |
| How to compile nearcore | Same as above | @marshall-d-teach | Jul 1 2020 | [Github](https://github.com/marshall-d-teach/near-doc/blob/master/create-validator.md) | 1,000 |
| How to compile nearcore | Same as the above | @imnisen | Jul 6 2020 | [Github](https://github.com/imnisen/near-stakewars-guide/blob/master/challenge001-how-to-compile-nearcore.org) | 1,000 |
| How to compile nearcore | Same as the above | @Lalit15 | Jul 21 2020 | [Medium](https://medium.com/@coolphil15/launch-near-node-on-near-betanet-test-network-4fcd810a71b5) | 1,000 |
| How to compile nearcore | Same as the above | @qwerspe | Jul 21 2020 | [Medium](https://medium.com/@qwerspe/compile-nearcore-and-create-validator-338fbf471ec0) | 1,000 |
| How to compile nearcore | Same as the above | @48cfu | Aug 24 2020 | [GitHub](https://github.com/48cfu/near-documentazione/blob/master/diventare-validatore.md) IT | 1,000 |
| How to compile nearcore | Same as the above | @mabalaru | Sep 17 2020 | [Medium](https://medium.com/@inotel/compile-nearcore-and-create-validator-6169c16838ee) EN | 1,000 |
| How to compile nearcore | Same as the above | @youlaiwuqu | Sep 7 2020 | [Jianshu](https://www.jianshu.com/p/d00033c83063) | 1,000 |
| How to compile nearcore | Ukrainian guide with video: сompile nearcore and deploy stacking pool | @Cryptomilion | Sep 23 2020 | [Medium](https://medium.com/@shiverov/%D0%B7%D0%B0%D0%BF%D1%83%D1%81%D0%BA-%D0%BD%D0%BE%D0%B4%D0%B8-nearcore-near-%D0%B2-%D1%82%D0%B5%D1%81%D1%82%D0%BE%D0%B2%D1%96%D0%B9-%D0%BC%D0%B5%D1%80%D0%B5%D0%B6%D1%96-betanet-216415954c4e) | 1,000 |
| Explain NEAR keys | Explain in the form of a video, a blogpost or Github document how `near login` works, how the authorization keys are generated, and how you can manage their backup and restore properly. Show in the guide the proof that you were able to run `near shell` on a different machine, without authorizing a new key. Contributions in other languages are encouraged, but considered on a case by case basis. Updates to this guide, reflecting any updates of the tools involved, will be paid a % of the initial bounty per each revision, up to once per month, until Stake Wars is over.  | @narniec | June 12 2020 | [Medium](https://medium.com/@narniec2020/near-shell-backup-%D0%BA%D0%BB%D1%8E%D1%87%D0%B5%D0%B9-b30e8ed1d39d); [Youtube](https://www.youtube.com/watch?v=VHtzBjbmzT0&feature=youtu.be) | 1,000 |
| Explain NEAR keys | Same as above | @wjdfx | June 15 2020 | [Jianshu](https://www.jianshu.com/p/f238b7049cb9) | 1,000 |
| Explain NEAR keys | Same as above | @georgec138 | June 23 2020 | [Github](https://github.com/georgec138/near-docs/blob/master/Explain-NEAR-keys_cn.md) | 1,000 |
| Explain NEAR keys | Same as above | @bonsfi | June 23 2020 | [Medium](https://link.medium.com/LQTQbsABy7) | 1,000 |
| Explain NEAR keys | Same as above | @marshall-d-teach | Jul 6 2020 | [Github](https://github.com/marshall-d-teach/near-doc/blob/master/explain-near-keys.md) | 1,000 |
| Explain NEAR keys | Same as above | @Viacheslav198 | Jul 7 2020 | [Github](https://github.com/Viacheslav198/near/blob/master/keys.md) | 1,000 |
| Explain NEAR keys | Same as above | @Vasya-krypto | July 21 2020 | [Medium](https://medium.com/@orang3club/explain-near-keys-737162914413) | 1,000 |
| Explain NEAR keys | Same as above | @Lalit15 | July 21 2020 | [Medium](https://medium.com/@coolphil15/how-to-generate-near-keys-using-near-shell-and-authorize-access-6b114003e9d) | 1,000 |
| Explain NEAR keys | Same as above | @48cfu | Aug 24 2020 | [GitHub](https://github.com/48cfu/near-documentazione/blob/master/chiavi-spiegate.md) IT | 1,000 |
| Explain NEAR keys | Same as above | @everuner | Sept 3 2020 | [Medium](https://medium.com/@serj_czech/explain-near-keys-845c9ad7b13d) | 1,000 |
| Explain NEAR keys | Same as above | @yes-filippova | Sept 13 2020 | [Medium](https://medium.com/@yes.filippova/%D0%B3%D0%B5%D0%BD%D0%B5%D1%80%D0%B0%D1%86%D1%96%D1%8F-%D0%BA%D0%BB%D1%8E%D1%87%D1%96%D0%B2-near-%D1%82%D0%B0-%D1%97%D1%85-%D0%B1%D0%B5%D0%BA%D0%B0%D0%BF-963cd6ab563?source=friends_link&sk=46b4d78cdfcf168234894e6df0fa9825) UA | 1,000 |
| Explain NEAR keys  | Same as above | @mabalaru | Sep 17 2020 | [Medium](https://medium.com/@inotel/explain-near-keys-c3bda285ebb2) EN | 1,000 |
## Next Challenge
Enroll your staking pool, receive your delegation, and maintain your validator status: [challenge002](challenge002.md)
