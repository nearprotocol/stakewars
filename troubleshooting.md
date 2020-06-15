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
2. I get a `type: 'UntypedError'` if I try to use near-shell
3. I get a `KeyNotFound` error if I try to use near-shell
4. I get a timeout error from RPC after I sent a command
5. I had a wallet created on [nearprotocol.com](http://nearprotocol.com) and now I can't see it anymore

### Staking-related issues
1. I used `near send` instead of `near call` to a staking pool
1. I used `near stake` instead of `near call` to stake funds to a pool
1. I get a `GuestPanic` when I try to _unstake_ funds from my staking pool
1. I get a `GuestPanic` when I try to _withdraw_ funds from my staking pool
1. I set up the wrong owner to my staking pool, are funds lost?

## Node-related issues

### 1. My validator is in the `current_validators` set, but it's not producing blocks
This issue can happen if there's a mismatch of `public_key` or `account_id` between your validator node and the staking transaction.

**On your node**

Check if your node is actively validating blocks, by inspecting the logs with the command `docker logs nearcore --tail 5 2>&1 | grep "V/"` or `nearup logs | grep "V/" -n 5` (depending if you are running nearup with or without docker). If this command produces zero results, your node may be misconfigured. 

Issue the command `cat .near/betanet/validator_key.json | grep "account_id\|public_key"`. You should expect the following result:
```
  "account_id": "c2.nearkat",
  "public_key": "ed25519:Zk6cdWPxmK1H5cy3K3GbyDRUyHz9w8a9Q2Mb7ezKiHW",
```
Note this result and proceed to verify the staking pool configuration.

**On your near-shell machine**

Issue the command `curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq -c '.result.current_validators[] | select(.account_id | contains ("c2.nearkat"))'`. (Remember to replace `c2.nearkat` with the account_id from your validator_key.json)

You should expect the following result:

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


### 2. I get Telemetry data errors in the logs
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

### 1. I get a `Server error: Timeout` when I use near-shell
If you are running on BetaNet, we often apply experimental features that can impact the stability of the RPC at the address `https://rpc.betanet.near.org`.

The error may look similar to this one:
```
Error:  TypedError: [-32000] Server error: Timeout
    at JsonRpcProvider.sendJsonRpc (/usr/lib/node_modules/near-shell/node_modules/near-api-js/lib/providers/json-rpc-provider.js:129:27)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
    at async Account.signAndSendTransaction (/usr/lib/node_modules/near-shell/node_modules/near-api-js/lib/account.js:97:22)
    at async scheduleFunctionCall (/usr/lib/node_modules/near-shell/commands/call.js:30:34)
    at async Object.handler (/usr/lib/node_modules/near-shell/utils/exit-on-error.js:4:9) {
  type: 'UntypedError'
}
```

There are three possible remetiations:
1. Look for the transaction on the [BetaNet Explorer](https://explorer.betanet.near.org)
2. Point near-shell to a local node
3. The network is not producing new blocks

**1. Look for the transaction on the BetaNet Explorer**
Most of the time, your transaction was correctly sent to the network, regardless of the timeout error in `near-shell`.
If you don't see the transaction in the explorer, you can issue the command again and refres the explorer page.

**2. Point near-shell to a local node**
Download and run `nearup` on your local machine, and point near-shell to the localhost for the queries. As an example, the command `near state test` would become `near state test --nodeUrl http://127.0.0.1:3030 --helperUrl http://127.0.0.1:3030`.
You should see a result similar to this one:

```
Using options: {
  nodeUrl: 'http://127.0.0.1:3030',
  helperUrl: 'http://127.0.0.1:3030',
  networkId: 'betanet',
  contractName: undefined,
  walletUrl: 'https://wallet.betanet.near.org',
  useLedgerKey: "44'/397'/0'/0'/1'",
  accountId: 'test',
  initialBalance: null
}
Account test
{
  amount: '78447050893413945551764297785000',
  locked: '0',
  code_hash: '11111111111111111111111111111111',
  storage_usage: 182,
  storage_paid_at: 0,
  block_height: 7095636,
  block_hash: '9ukamhGe6fFgDnHXWvUQJoJhWXVQqKxqEr7zmmugfyWs',
  formattedAmount: '78,447,050.893413945551764297785'
}

```

**3. The network is not producing new blocks**
In the unlikely situation that NEAR Protocol is not producing new blocks, near-shell cannot issue commands that change the state of the ledger, such as `near call` or `near login`. In this case, you have to check the status of the network from the page at https://status.nearprotocol.com/ and wait when the services are back online to perform such actions.

### 2. I get a `type: 'UntypedError'` if I try to use near-shell
Most of the time you can find the reason of the error by scrolling up a few lines:
```
Using options: {
  networkId: 'default',
  nodeUrl: 'https://rpc.testnet.near.org',
  contractName: undefined,
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  useLedgerKey: "44'/397'/0'/0'/1'",
  accountId: 'nearkat',
  initialBalance: null
}
Error:  TypedError: [-32000] Server error: account nearkat does not exist while viewing
    at JsonRpcProvider.sendJsonRpc (/usr/lib/node_modules/near-shell/node_modules/near-api-js/lib/providers/json-rpc-provider.js:129:27)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
    at async JsonRpcProvider.query (/usr/lib/node_modules/near-shell/node_modules/near-api-js/lib/providers/json-rpc-provider.js:61:24)
    at async Account.fetchState (/usr/lib/node_modules/near-shell/node_modules/near-api-js/lib/account.js:46:23)
    at async Account.state (/usr/lib/node_modules/near-shell/node_modules/near-api-js/lib/account.js:53:9)
    at async Near.account (/usr/lib/node_modules/near-shell/node_modules/near-api-js/lib/near.js:41:9)
    at async exports.viewAccount (/usr/lib/node_modules/near-shell/index.js:161:19)
    at async Object.handler (/usr/lib/node_modules/near-shell/utils/exit-on-error.js:4:9) {
  type: 'UntypedError'
}

```
The last line is not important, and you want to analyze the `Error:` line of the message. In this case, the error is `account nearkat does not exist while viewing`, so as a user you have to double-check the spelling of the account - or verify that you are looking on the right network as here I'm trying to find the user `nearkat` on TestNet and not BetaNet.


### 3. I get a `KeyNotFound` error if I try to use near-shell
Your near-shell may not have the keys to operate on your account, showing an error similar to the one below: 

```
}
Sending 1 NEAR to test from bowen.test
Error:  TypedError: Can not sign transactions for account bowen.test, no matching key pair found in Signer.
    at Account.signAndSendTransaction (/usr/local/lib/node_modules/near-shell/node_modules/near-api-js/lib/account.js:91:19)
    at async exports.sendMoney (/usr/local/lib/node_modules/near-shell/index.js:198:33)
    at async Object.handler (/usr/local/lib/node_modules/near-shell/utils/exit-on-error.js:4:9) {
  type: 'KeyNotFound'
}
```

**Remediations**
You can use `near login` again, and authorize your machine, or just copy your json key from your backups to the `~/.near-credentials/betanet/` folder. Previous versions of near-shell were storing credentials in the `neardev` folder, so if your account name is `nearkat.betanet` you may look for it with the command `find -name "*nearkat*.json"`, showing results as below:
```
./betanet/c1.nearkat.betanet.json
./betanet/nearkat.betanet.json
./neardev/betanet/c2.nearkat.betanet.json
./neardev/betanet/c3.nearkat.betanet.json
./neardev/betanet/c1.nearkat.json
./neardev/betanet/c1.nearkat.betanet.json
./neardev/betanet/c2.nearkat.json
./neardev/betanet/nearkat.betanet.json
./neardev/betanet/nearkat.json
./.near-credentials/betanet/c2.nearkat.json
```
The files `./betanet/nearkat.betanet.json` and `./neardev/betanet/nearkat.betanet.json` contain a valid private key to control the account `nearkat.betanet`, so moving them to the directory `./.near-credentials/betanet/` will allow your near-shell to sign transaction for that account.

**Heads Up:** This solution doesn't apply if you are using a Ledger Wallet.

### 4. I get a timeout error from RPC after I sent a command
NEAR RPC may be unresponsive or too slow to reply, generating this type of error. The first step is to control that all the services are running correctly from the url https://status.nearprotocol.com/.

If all services are operational, try to use the explorer yourself, by visiting the url https://explorer.betanet.near.org. If the explorer itself is slow or unresponsive, it's possible that your RPC calls need a longer timeout.

**Remediation:**
You can call the same RPC commands to your local node, replacing `https://rpc.betanet.near.org` with the IP address of your node. As an example the command
```
curl -d '{"jsonrpc": "2.0", "method": "status", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq
```
becomes
```
curl -d '{"jsonrpc": "2.0", "method": "status", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' http://127.0.0.1:3030 | jq
```
(note that you have to use plain http, and specify the port `3030`).


### 5. I had a wallet created from the URL at betanet.nearprotocol.com and now I can't see it anymore
NEAR Protocol recently switched from the website `nearprotocol.com` to `near.org`. As a result, some cookies have been reset, and you can't see your wallet anymore because it's now moved from wallet.nearprotocol.com to wallet.near.org.

**Remediation:**
You have two main ways to recover your account:
1. Use the method above to recover your json wallet file, and copy it where it is needed
2. Recover your wallet using the recovery email, sms or seed passphrase

While the option 1 is already documented above, option 2 requires additional steps if you want to use the recovery email with a _magic link_. More specifically, the magic link may still point to `nearprotocol.com`, so your recovery could fail.
Look for any email coming from `wallet@nearprotocol.com` and copy/paste the "Recover Account" link on your favorite text editor.
The result should be similar to the one below:
```
http://undefined/recover-with-link/nearkat/crazy%20horse%20battery%20staple
```
Your seed passphrase is separated by `%20`, so you can manually separate the keys (in this case they are `crazy`, `horse`, `battery`, `staple`) and try to recover your account using the passphrase metod.

**Heads Up:** always use the right wallet address to recover your account:
| Network | Wallet URL |
| ------- | ---------- |
| MainNet | https://wallet.near.org |
| TestNet | https://wallet.testnet.near.org |
| BetaNet | https://wallet.betanet.near.org |
| DevNet | https://wallet.devnet.near.org |

Trying to use the right passphrase with the wrong wallet URL will produce no results.


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