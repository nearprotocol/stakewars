# Stake Wars Challenge 004
Published on: June 22 2020

Create a warchest of staked tokens, and dynamically maintain **no more than one seat**.
This challenge is designed to learn how to monitor the minimum stake to become a validator, and dynamically manage your staking pool. Your achievement will be to control a liquid reserve of tokens, the _warchest_, that you can leverage if your validator risks to be kicked out from the current set for insufficient stake.
**Heads Up:** After you complete the script, fill up this form: https://nearprotocol1001.typeform.com/to/x4Bval

## Acceptance Criteria

1. Monitor your stake
2. Manage the seat price

## 1.Monitor your stake
Use [near-cli](https://github.com/near/near-cli) or the `validators` method in the [JSON RPC](https://docs.near.org/docs/interaction/rpc) to query the validators state:
| Action | near-cli | validators JSON RPC |
| ------ | ---------- | -------- |
| current set (t0) | `near validators current` | `result.current_validators` |
| next set (t+1) | `near validators next` | `result.next_validators` |
| proposals (t+2) | `near proposals` | `result.current_proposals` |

Where `t0` is the current epoch, and `t+n` epochs in the future.

### Monitor the current set of validators with near-cli
Use
```
near validators current | awk '/<POOL_ID>/ {print $4}'
```
This command will generate a string with an integer in NEAR tokens, where:
- `near validators current` is used to show the current set of validators
- `awk '/<POOL_ID> {print $4}'` filters by POOL_ID, and prints an integer with its current stake

### Monitor the current set with the RPC
Use
```
curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq -c '.result.current_validators[] | select(.account_id | contains ("<POOL_ID>"))' | jq .stake
```
This command query the JSON RPC with:
- `"method": "validators"`
- `jq -c '.result.current_validators` to visualize only the current set
- `select(.account_id | contains ("<POOL_ID>"))'` to filter only <POOL_ID> from the results
- `jq .stake` to filter again via jq the results and take only the total stake in YoctoNEAR

If compared with near-cli, this query provides a more accurate stake of the <POOL_ID>.

You can use similar filters to check if your pool will be in the next set or not:

### Monitor the next set with near-cli
Use the command below to see if your node will lose its seat in the next epoch:
```
near validators next | grep "Kicked out" | grep "<POOL_ID>"
```
If the result is not empty, <POOL_ID> will not be in the next validators set, and will lose its seat.

Alternatively, you can use
```
near proposals | grep "Rollover" | grep "<POOL_ID>"
```
If this result is not empty, <POOL_ID> will be in the Rollover set, and will maintain the validator seat.


### Monitor the next set with the JSON RPC:
Similar to the commands above, use
```
curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq -c '.result.next_validators[] | select(.account_id | contains ("<POOL_ID>"))'
```
If the result is not empty, <POOL_ID> will be in the next validators set.

The RPC provides data on the previous epoch, to investigate the reason why a node is not in the current set:
```
curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq -c '.result.prev_epoch_kickout[] | select(.account_id | contains ("<POOL_ID>"))' | jq .reason
```
Similar to the other command above above:
- `jq -c '.result.prev_epoch_kickout` filters the previous epoch set kick out
- `jq .reason` filters the reason, eg insufficient stake or insufficient number of blocks generated

### Monitor the epoch progress
- get the current block height
- get the current epoch start

As an example, you can use the command
```
curl https://rpc.betanet.near.org/status | jq .sync_info.latest_block_height
```
to return an integer of the current block height.

As of today, you can retrieve the `epoch_start` from the JSON RPC:
```
curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq .result.epoch_start_height
```
This query will generate an integer with the block number that started the current epoch

As a final step, estimate how many blocks are left in the current epoch by subtracting the `latest_block_height` from `epoch_start_height + 10000`.

**Heads up:** BetaNet, TestNet and MainNet have different epoch lengths:

| Network | Epoch Blocks |
| ------- | ------ |
| BetaNet | 10,000 |
| TestNet | 43,200 |
| MainNet | 43,200 |

### Monitor the seat price
Measure or calculate yourself the cost of a seat to become validator.

As an example, you may use near-cli to know:
- the current epoch seat price with `near validators current | awk '/price/ {print substr($6, 1, length($6)-2)}'`
- the next epoch seat price price with `near validators next | awk '/price/ {print substr($7, 1, length($7)-2)}'`
- the estimated t+2 seat price with `near proposals | awk '/price =/ {print substr($15, 1, length($15)-1)}'`


## 2.Manage the seat price
This challenge is complete when you can dynamically adjust the locked balance of your staking pool, and maintain **one seat**.

You can use the commands `stake` and `unstake` with near shell, dynamically locking your funds:
```
near call <POOL_ID> stake '{"amount": "<STAKE_AMOUNT>"}' --accountId <WARCHEST_ID>
```
Where:
- `POOL_ID` is the name of your staking pool
- `STAKE_AMOUNT` is calculated from the data collected above
- `WARCHEST_ID` is the account that you use to delegate funds to your pool

If your current stake provides two seats or more, your funds should be unstaked and held in the warchest balance.

To prove that your _warchest_ is deployed, provide a list of 4 transactions that staked or unstaked funds based on the seat pric following these principles. Reply to this [Form](https://nearprotocol1001.typeform.com/to/x4Bval) to receive 100,000 extra betanet tokens delegated to your pool and begin the challenge.

**Heads up:** we will run our monitoring scripts too, and will un-delegate the 100,000 tokens from your pool if:
- you retain **two or more seats** for three epochs in a row
- you retain **two or more seats** for 15 epochs in total

Overall, we suggest to use NEAR's delegated tokens as your main stake in the pool, and keep your own token reserves (earned from previous challenges and contributions) as liquid tokens within the pool.
Once you are running your _warchest_, reply to the [Issue #500](https://github.com/nearprotocol/stakewars/issues/500) posting your accomplishment. Use also the same issue if we undelegated your funds, and you want to try again.

## Contribution Opportunities

Do you want to earn extra tokens? We have contribution opportunities available below! 

Reply to the challenges application thread on [Github](https://github.com/nearprotocol/stakewars/issues/350) specifying:
- which contribution you want to do, and the link to this challenge
- the type of document you will release
- the date when you will publish the content (you can postpone a few days, no worries)

Once your work is done, you will be added to the list below. Please note that rewards in tokens will need to pass basic KYC checks from NEAR Foundation, and comply with regulations.

### List of available contributions

| Abstract | Description                    | Contributor |  Date  | Link | NEAR Tokens | Maintenance | Language |
| -------- | ------------------------------ | ----------- | ------ | ---- | ----------- | --- | ---- |
| Monitor the Stake | Create a tutorial, in the form of a video, a blogpost or Github document, that shows how to monitor your stake, and the current seat price. The goal is to help users integrate this system with their monitoring platform, such as Grafana or Datadog. Updates to this guide, reflecting any updates of the tools involved, will be paid a % of the initial bounty per each revision, up to once per month, until Stake Wars is over. Contributions in other languages are encouraged, but considered case by case basis. | @narniec | Jun 23 2020 | [Medium](https://link.medium.com/ycHhCnWBy7 ) | 1,000 | 15% | RU |
| Monitor the Stake | Same as the above | @masknetgoal634 | Jun 26 2020 | [Github](https://github.com/masknetgoal634/near-go-warchest/blob/master/README.md) | 1,000 | 15% | EN |
| Monitor the Stake | Same as the above | @bonsfi | Jul 21 2020 | [Medium](https://medium.com/@bonsfi/como-monitorear-el-stake-de-tu-validador-en-near-protocol-d709326cf6ff?sk=09f62413b029762a549b5596c9543a4c) | 1,000 | 15% | ES |
| Release the Warchest Bot | Same as the above | @masknetgoal634 | Jun 26 2020 | [Github](https://github.com/masknetgoal634/near-go-warchest) | 2,500 | 15% | EN |
| Release the Warchest Bot | Release a Warchest Bot, in your favorite programming language, capable to manage your validator seat and maintain its number to **one**. It doesn't have to be production-ready, but it should document how to install and run it. | @48cfu | Jun 28 2020 | [Github](https://github.com/48cfu/near-warchest-bot) | 2,500 | 15% | EN |
| Release the Warchest Bot | Same as the above | @gaia | Jun 29 2020 | [Github](https://gist.github.com/gaia/cff45baf3fa710a42c3fc4cdaafe8edc) | 2,500 | 10% | EN |
| Release the Warchest Bot | Same as the above | @eorituz | Jul 1 2020 | [Github](https://github.com/eorituz/near_warchest) | 2,500 | 10% | EN |
| Release the Warchest Bot | Same as the above | @imnisen | Jul 6 2020 | [Github](https://github.com/imnisen/near-warchest/) | 2,500 | 15% | EN |
| Release the Warchest Bot | Same as the above | @WilliamGuozi | Jul 21 2020 | [Github](https://github.com/WilliamGuozi/near-monitor) | 2,500 | 15% | EN |
| Release the Warchest Bot | Same as the above | @minstr22 | aug 28 2020 | [Github](https://github.com/minstr22/Near-Protocol-warchest) | 2,500 | 15% | EN |
| Install and start warchest bot | Explain installation, configuration and monitoring work of warchest bot by @eorituz. Process of obtaining results | @everuner | Aug 30 2020 | [Medium](https://medium.com/@serj_czech/near-warchest-%D0%B1%D0%BE%D1%82-stake-wars-challenge-004-a332d028fb53) | - | - | RU |

## Next Challenge
Automatically deploy nearcore using a CI/CD pipeline: [challenge005](challenge005.md)

## Previous Challenge
Monitor your node health, and setup automated email alerts: [challenge003](challenge003.md)
