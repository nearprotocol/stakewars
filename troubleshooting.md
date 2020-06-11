# Stake Wars Troubleshooting

This document will try to address the most common issues that you may have while running Stake Wars and setting up your node.
Contributions and corrections are welcome!

## Table of contents

### Node-related issues
1. My validator is in the `current_validators` set, but it's not producing blocks
2. I get `Telemetry data` error in the logs
3. I get a `Peer stream error` while syncing with other nodes
4. My node seems stuck at 98% of the sync, without progressing
5. My node is stuck at zero peers

### Near-shell and RPC issues
1. I get a `Server error: Timeout` when I use near-shell
1. I get a `type: 'UntypedError'` if I try to use near-shell
1. I get a `KeyNotFound` error if I try to use near-shell
1. I get a timeout error from RPC after I sent a command
1. I had a wallet created on [nearprotocol.com](http://nearprotocol.com) and now I can't see it anymore

### Staking-related issues
1. I used `near send` instead of `near call` to a staking pool
1. I used `near stake` instead of `near call` to stake funds to a pool
1. I get a `GuestPanic` when I try to _unstake_ funds from my staking pool
1. I get a `GuestPanic` when I try to _withdraw_ funds from my staking pool
1. I set up the wrong owner to my staking pool, are funds lost?

## Node-related issues

### 1. My validator is in the current_validators set, but it's not producing blocks
This issue can happen if there's a mismatch of `public_key` or `account_id` between your validator node and the staking transaction.

**On your node**

Issue the command `cat .near/betanet/validator_key.json | grep "account_id\|public_key"`. You should expect the following result:
```
  "account_id": "c2.nearkat",
  "public_key": "ed25519:Zk6cdWPxmK1H5cy3K3GbyDRUyHz9w8a9Q2Mb7ezKiHW",
```

**On your near-shell machine**

Issue the command `curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq -c '.result.current_validators[] | select(.account_id | contains ("c2.nearkat"))'`. You should expect the following result:

```
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 39719  100 39641  100    78   342k    690 --:--:-- --:--:-- --:--:--  343k
{"account_id":"c2.nearkat","public_key":"ed25519:Zk6cdWPxmK1H5cy3K3GbyDRUyHz9w8a9Q2Mb7ezKiHW","is_slashed":false,"stake":"289982335519735189330363217693","shards":[0],"num_produced_blocks":196,"num_expected_blocks":197}
```
If the `account_id` or `public_key` are different from your validator_key.json, your pool is misconfigured and you have to apply some changes.

**Remediation**

If your `account_id` is wrong, follow these steps:
1. Stop your node with `nearup stop`
2. Edit the file `~/.near/betanet/validator.json` and manually change the `account_id` with the one from the RPC query
3. Delete the data folder with the command `rm -rf ~/.near/betanet/data`
4. Restart your node

You might expect a result similar to this one:
```
Using officially compiled binary
Downloaded binary version is up to date
Our genesis version is up to date
Stake for user 'c2.nearkat' with 'ed25519:Zk6cdWPxmK1H5cy3K3GbyDRUyHz9w8a9Q2Mb7ezKiHW'
Starting NEAR client...
Node is running! 
To check logs call: `nearup logs` or `nearup logs --follow`
```
If successful, `num_produced_blocks` will be higher than zero.

**Otherwise**, if your `public_key` is wrong, follow these steps:
1. Issue the command `near call <ACCOUNT_ID> update_staking_key '{"stake_public_key":"ed25519:Zk6cdWPxmK1H5cy3K3GbyDRUyHz9w8a9Q2Mb7ezKiHW"}' --accountId <OWNER_ID>`
2. Issue the command `curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq -c '.result.current_proposals[] | select(.account_id | contains ("<ACCOUNT_ID>"))'`

(where <ACCOUNT_ID> is the ID of your staking pool, and <OWNER_ID> is your main wallet).

You might expect a result similar to this one:
```
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 33922  100 33844  100    78   367k    866 --:--:-- --:--:-- --:--:--  368k
{"account_id":"c2.nearkat","public_key":"ed25519:Zk6cdWPxmK1H5cy3K3GbyDRUyHz9w8a9Q2Mb7ezKiHW","stake":"101665314574769168872143240913"}
```
If successful, `num_produced_blocks` will be higher than zero.

Remember to ping your staking pool to re-issue your staking transaction, as your node will most probably be kicked out in the next epoch, and you will lose your validator seat. Use the command `near call <ACCOUNT_ID> ping '{}' --accountId <OWNER_ID>`.


### 2. I get _Telemetry data_ errors in the logs
Sometimes, after you restart your node or shortly after an update, you may see this error in the logs:
```
Jun 10 23:17:30.014  INFO telemetry: Telemetry data could not be sent due to: Failed to connect to host: Timeout out while establishing connection
```

This is not an issue, you can safely ignore this error as it only impacts the ability of the BetaNet explorer to receive data from your node. As a result, it is possible that your node won't appear in the list at the address https://explorer.betanet.near.org/nodes/validators.


### 3. I get a `Peer stream error` while syncing with other nodes
If you see an error similar to this one:
```
May 26 04:39:58.712 WARN network: Peer stream error: Os { code: 104, kind: ConnectionReset, message: "Connection reset by peer" }
```
it is possible that one of your peers disconnected your node. It may happen if:
- your node is not running the same version of the other peer
- your node is a validator, but it's producing zero blocks (see above)

If this message is repeated multiple times and your node gets zero connected peers, you may need to stop nearup, clean entirely the data in the folder at `~/.near/betanet/`, and start the node again.

### 4. My node seems stuck at 98% of the sync, without progressing
The counter simply measures the number of blocks remaining to reach the tip of the chain, and not the amount of computation necessary to complete the operation. As a result, is possible that certain blocks contain more information and need more time to be processed, showing slower progress.

If your node has no active peers, you may need to reset your `.near/betanet` folder, and generate a new node key. Check the in the logs if the number of peers is not `0/0/40`:

```
Jun 10 23:18:15.092  INFO stats: # 6949467 Downloading headers 5% -/86  8/8/40 peers ⬇ 493.4kiB/s ⬆ 181.2kiB/s 0.00 bps 0 gas/s CPU: 80%, Mem: 462.5 MiB
```

### 5. My node is stuck at zero peers
It is possible that your node was banned by other peers because it is running an outdated release of `nearcore`.

**On your node**
Issue one of these two commands:
- `docker logs nearcore 2>&1 | grep Build` if you are running nearup using Docker
- `nearup logs | grep Build` if you are running nearup with the --nodocker switch

You should expect a result similar to this one:
```
Jun 10 23:17:11.628  INFO near: Version: 1.0.0, Build: b30864b8, Latest Protocol: 22 
```

**On your near-shell machine**
Issue the command `curl -d '{"jsonrpc": "2.0", "method": "status", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq .result.version`

You should expect a result similar to this one:
```
{
  "version": "1.0.0",
  "build": "b30864b8"
}
```

If you see any differences, your node is running an outdated version of nearcore and needs to be reset manually. 

**Remediation**
1. Stop nearup with `nearup stop`
2. Make a backup copy of your validator_key.json file with the command `cp ~/.near/betanet/validator_key.json ~`
3. Clean the betanet folder with the command `rm -rf ~/.near/betanet`
4. [ONLY FOR DOCKER USERS] run the command `docker system prune --volumes` to clean up the containers and the cache
5. Start nearup again with the command `nearup betanet` (remember to add `--nodocker` and the `--binary-path` if you are using them)
6. Press enter, leaving empty the validator ID
7. Stop nearup again with `nearup stop`
8. Clean the data folder in `~/.near/betanet/data`
9. Copy the validator key in the betanet folder with the command `cp ~/validator_key.json ~/.near/betanet/`
10. Start nearup again 

At the end of this process, you should expect a message like the one below:

```
Using officially compiled binary
Downloaded binary version is up to date
Our genesis version is up to date
Stake for user 'c2.nearkat' with 'ed25519:Zk6cdWPxmK1H5cy3K3GbyDRUyHz9w8a9Q2Mb7ezKiHW'
Starting NEAR client...
Node is running! 
To check logs call: `nearup logs` or `nearup logs --follow`
```

This process will use nearup scripts to generate the config files, the node keys and the genesis.json file from scratch, while keeping the validator_key intact (which is used by the staking pool contract).
Be sure that only one node at a time is using your validator_key.json, or you risk to double-sign blocks.

## Near-shell and RPC issues

_Work in Progress, stay tuned_

### 1. I get a KeyNotFound error if I try to use near-shell

```
{
  type: 'KeyNotFound'
}
```



## Staking-related issues

_Work in Progress, stay tuned_


### 1. I get a GuestPanic error when I try to _unstake_ funds from my staking pool

```
{
  type: 'GuestPanic',
  index: 0,
  panic_msg: "panicked at 'The new total balance should not be less than the old total balance', src/lib.rs:509:9"
}
```

### 1. I get a GuestPanic when I try to _withdraw_ funds from my staking pool

```
{
  type: 'GuestPanic',
  index: 0,
  panic_msg: "panicked at 'Not enough unstaked balance to withdraw', src/lib.rs:224:9"
}
```

## Additional resources

- Join the Validator chat at [https://near.chat].
- Ask questions on Stack Overflow, using the tags [nearprotocol](https://stackoverflow.com/questions/tagged/nearprotocol) and [nearprotocol-validator](https://stackoverflow.com/questions/tagged/nearprotocol-validator).
- Submit a bug (or ask for a new feature!) directly from the Stake Wars [issues section](../../issues).