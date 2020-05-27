# Stake Wars Challenge 002
Published on: May 25th 2020

Enroll your staking pool, receive your delegation, and keep your validator status!
This challenge is designed to learn how to keep track of your validator, and make your delegators happy.


## Acceptance Criteria

1. Enroll your staking pool
2. Receive your delegation
3. Check your validator status
4. Collect your fees!

## 1.Enroll your staking pool
As soon as you completed the Stake Wars [Challenge 001](challenge001.md), you are ready to join the roster of other professional validators, and receive your first delegation. 
The process will need some basic expertise with `git`. Follow the instructions in the file [VALIDATORS.md](../VALIDATORS.md), and submit a pull request to the branch `update-validators-list`. 

## 2.Receive your delegation
NEAR team, and other Stake Wars participants, will keep an eye on the validators who deployed the staking pool, and will delegate some tokens to the ones of their choice. At the moment of writing, the cost of a validator seat on BetaNet is higher than 50,000 NEAR tokens, and there are only 100 seats available. With more than 120 validators already on the list (and growing), you may need to spend some time to make your profile nice and attractive.

This step of the challenge requires that you have at least 100 tokens alread delegated to your staking pool. If you participated in the previous Stake Wars, and you already received BetaNet tokens in the past weeks, you can delegate them to your own pool, and "skip the line".

To make this more challenging, understand how to use the comamnd `near delete-key` from near-shell, and enable the lock on your staking pool. Be careful: once you lock your contract, you can only change the validator key and the fees, but not its name.

## 3.Check your validator status
You have five different ways to verify that your staking pool is successfully working, and you are a validator:
1. Check on the betanet block explorer
2. Use the `near validators next` on near-shell
3. Control the locked balance in your account, using `near state` command
4. A change in the `nearup` logs displayng an uppercase "V"
5. A query to the JSON RPC using the `validator` endpoint

### 3.1.Check on the betanet block explorer
Visit the [betanet explorer](https://explorer.betanet.near.org) and check that your staking transaction succeeded:
![alt text](../media/near_explorer_staking_pool.png "Explorer successful transaction")

### 3.2.Use near-shell
Use the command `near validators current` and `near validators next` to know who is currently running the network, a who will join next.
The command `near validators next | grep 'seat price'` will tell you how big is the stake necessary to become a validator.

### 3.3.Control the locked balance
Use the command `near state stakingPool_ID` to see the locked balance in your staking pool, and control if it is sufficient to earn a seat as a validator.

### 3.4.Check your nearup logs
Nearup logs can show if you are a validator or not. The `V/71` below shows that your node is one of the 71 validators running the network.
![alt text](../media/nearcore_logs_wide.png "nearcore validator logs")

### 3.5.Query the JSON RPC
A combination of `curl` and `jq` can give you a trove of information about present and future validators:

```bash
curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org -v | jq 
```

The filter `jq .result.current_proposals` should show a staking transaction issued by your staking pool.

### 4.Collect your fees!

If you configured your pool correctly, and you control the pool owner wallet, you should be able to unstake and withdraw the fees collected by your pool. These fees will be listed as normal stake in the account used as the `owner` of the staking pool.


## HEADS UP!

Always check that your `stakingPool_ID` and your validator node have the same account_ID and public_keu. Otherwise, you will receive your delegation, your validator will receive a seat, but your node will not be able to create blocks! This is the best way to lose delegators, and miss the opportunity to complete this challenge.

If you issue the command `near validators current | grep ' 0%'` you will see that some validators got a seat, but generated 0% of the blocks. This means they will be kicked out in the next epoch (as you can see in `near validators next | grep 'Kicked'`), and their delegators will lose rewards.

Use the command `cat ~/.near/betanet/validator_key.json | grep 'public_key\|account_id'` and verify they are **exactly the same** of current proposals (see `3.5` above). NEAR team will delegate to your node some tokens, but if you misconfigure your pool and you produce 0 blocks, the tokens will be unstaked, and you will lose your priority.

## Contribution Opportunities

Do you want to earn extra tokens? We have contribution opportunities available below! 

Reply to the challenges application thread on[Github](https://github.com/nearprotocol/stakewars/issues/350) specifying:
- which contribution you want to do, and the link to the challenge
- the type of document you will release
- the date when you will publish the content (you can postpone a few days, no worries)

Once your work is done, you will be added to the list below. Please note that rewards in tokens will need to pass basic KYC checks from NEAR Foundation, and comply with regulations.

### List of available contributions

| Abstract | Description                    | Contributor | Due Date | Link | NEAR Tokens | Maintenance | Languange |
| -------- | ------------------------------ | ----------- | -------- | ---- | ----------- | --- | ---- |
| Enroll your node | Create a tutorial, in the form of a video, a blogpost or Github document, that shows how to do a pull request to the VALIDATORS.md file, lock your contract, delegate some tokens, become a validator, and check that your pool and your node are correctly configured. Contributions in other languages are encouraged, but considered on a case by case basis. Updates to this guide, reflecting any updates of the tools involved, will be paid a fraction of the initial bounty per each revision, up to once per month, until Stake Wars will be over. | - | - | - | 1,000 | 10% | - |
| Manage the pool | Create a tutorial, in the form of a video, a blogpost or Github document, that shows how to update the parameters of the staking pool, such as the owner id, the fees and the validator key. Show also how to collect the fees and unstake them. Contributions in other languages are encouraged, but considered on a case by case basis. Updates to this guide, reflecting any updates of the tools involved, will be paid a fraction of the initial bounty per each revision, up to once per month, until Stake Wars will be over. | - | - | - | 1,000 | 10% | - |
| Monitor uptime | Create a tutorial, in the form of a video, a blogpost or Github document, that shows how to automatically check the performance of your validator node, create reports (per epoch, per day) and send alerts via chat-bots or SMS if your node is stuck/kicked out and needs attention. Contributions in other languages are encouraged, but considered on a case by case basis. Updates to this guide, reflecting any updates of the tools involved, will be paid a fraction of the initial bounty per each revision, up to once per month, until Stake Wars will be over. | - | - | - | 1,000 | 10% | - |
| Use Ledger Nano S | Create a tutorial, in the form of a video, a blogpost or Github document, that shows how to run `near shell` with your Ledger Nano S, from the seed key creation, to its backup and restore, to the delegation of funds to a staking pool. Contributions in other languages are encouraged, but considered on a case by case basis. Updates to this guide, reflecting any updates of the tools involved, will be paid a fraction of the initial bounty per each revision, up to once per month, until Stake Wars will be over. | - | - | - | 1,500 | 10% | - |

