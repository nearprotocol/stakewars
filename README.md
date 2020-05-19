# **May 19th 2020 update: Stake Wars Episode II, Return of the Validators**
Stake Wars Episode II is officially live, see the [blog post](https://near.org/blog/stake-wars-episode-ii/) and the official [website page](https://near.org/stakewars/).
We learned a lot from running together the first phase of the Stake Wars, and we worked hard to release a NEAR node that is stable and production-ready.

We are now ready to get into the next step: open Stake Wars to anyone!

Stake Wars Episode II will test the *delegation* of tokens from users to validators. Therefore, we want you validators to be ready for the action and involve you with NEAR Protocol community as a whole.

The goals are:
- for end-users, learn the principles of proof of stake, and build confidence with the concepts of delegation
- for validators, integrate their platforms with NEAR's delegation smart contract, and build customer relationships early on
- for NEAR Protocol, iterate on the reference smart contract, and kickstart the community efforts to transform delegation into a new class of DeFi products


If you haven't already:
1. Join our Telegram validator channel [here](https://t.me/joinchat/Msj7FBRzcys3zb6J79u5bw) for general discussion
2. Join our Discord channel [here](https://discord.gg/ZMPr3VB) for troubleshooting and technical discussion

## The basics
If you are just curious, follow the quickstart guides below, and you'll be ready in a few minutes. However, this is just a shortcut to becoming familiar with high-level concepts of the protocol, not the most secure and efficient way to do it! 

### Quickstart For Validators
- Use [this form](https://forms.gle/5KabPsD4BefR6nv68) to enroll in the Stake Wars 2.0 and receive a few tokens to stake
- If you don't already have one, create a betanet account using our [hosted betanet wallet](https://wallet.betanet.nearprotocol.com)
- Spin up an Ubuntu/Debian VPS and follow the [nearup](https://github.com/near/nearup) tutorial
- Deploy your Staking Pool contract, from NEAR [Initial Contracts](https://github.com/near/initial-contracts/) repo
- Add your validator info to [this file](./VALIDATORS.md) - just fork and pull request, we'll merge ASAP

**HEADS UP:** We will reset Betanet every Tuesday at 6pm Pacific (Wednesday 0:00 UTC), so plan a node restart to download and install the new release automatically.

### Quickstart For Delegators (work in progress)
By now, we keep end-user delegation disabled. First we want to have a few validators offering a staking pool, and then we will switch to a larger scope of users.

**Heads up:** As a Delegator, you don't have to read further, this repository is written for node operators who want to become NEAR validators and enroll users like you. By now, we don't have any activity for non technical users, but stay tuned, we got news coming!

## Validator Walkthrough guide
If you plan to participate in the Stake Wars, we suggest you to follow this guide and join us on Discord if you have comments or issues.

This guide will help you to:

1. Generate a new keypair
2. Receive some tokens
3. Run nearcore via nearup
4. Stake some funds
5. Join Stake Wars Episode II

You can find a video explaining these steps also on [YouTube](https://youtu.be/MBLMzFKr1kg).

### 1.Generate a new keypair
You have to install [near-shell](https://github.com/nearprotocol/near-shell), a Node.js application that relies on nearlib to generate secure keys, connect to the NEAR platform and send transactions to the network on your behalf.
You don't have to install near-shell on your node - on the contrary, it's suggested to keep your staking wallet on a different machine.

1. `cd` into a directory you'd like to work in
2. Download `near-shell` with `npm install -g near-shell`
3. Set your NODE_ENV to betanet. You can do this in `bash` with the command `export NODE_ENV=betanet`
4. Create a work folder for the Stake Wars: `mkdir stakewars && cd stakewars`
5. Run `near login` and follow the instructions that take you to the wallet
6. Create your Staking Pool account, with the command `near create_account my_validator --masterAccount=owner --initialBalance 250` (see more documentation about near-shell [here](https://docs.near.org/docs/development/near-clitool)) - where `my_validator` is the account ID for your staking pool contract, `owner` is the account you used to perform the login at the step before, and `--initialBalance` is the amount in NEAR needed to pay the gas and deploy the smart contract
7. Once you created your staking pool account, `near-shell` is ready to send staking transactions

**Important** save the account ID you just created, you will need it to start the node in validator mode, and to deploy the staking pool contract. In the steps below, where you see `c1.nearkat` you will have to use the staking pool account, and where you see `nearkat.betanet` you will have to use your master account.

### 2.Receive some tokens 
This is a work in Progress. By now use this [request form](https://docs.google.com/forms/d/1xarv54e-fFSuD2AQorAPx4086z3zyS5ZNGwcLr4QkeQ). On the last page you can put your betanet account id. Starting May 18th 2020, tokens will be delegated to validators who deploy the staking pool contract. Only on a case-by-case basis, we may provide additional BetaNet tokens to test large-scale validation on betanet, without using the staking pool contract.

### 3a.Download nearup and go automatic
Before you start, please make sure that you have or setup a machine with a publicly routable ip address.
Use [nearup tutorial](https://github.com/near/nearup) to download the binaries and run your node automatically.

This process is not intended for production-grade deployments, and validators who may want to compile nearcore from its source code. Deploying `nearcore` from its source code is a key criteria to join other validators on MainNet Restricted. 

### 3b.Compile nearcore from source code
You can find updated build instructions on [NEAR docs](https://docs.nearprotocol.com/docs/local-setup/running-testnet#compile-testnet-without-docker), in the 'running testnet' section.
**Important:** instead of `testnet` you have to run `betanet`, therefore you have to download and compile the [beta branch](https://github.com/nearprotocol/nearcore/tree/beta), otherwise your node will not be able to connect to the network.

### 4. Stake funds with a delegation smart contract
This process is similar to the previous staking transaction, with the difference that you have to deploy the Staking Pool smart contract, and delegate your funds through it. Once the contract is deployed, any user will be able to deposit funds into your staking contract, and delegate their stake to your validator node.

More details are in the [Staking Pool contract](https://github.com/near/initial-contracts) repository.

Step-by-step guide (courtesy of [htafolla](https://github.com/htafolla)) for validators that are already staking, and want to deploy the staking pool the first time (or update it):

#### 4.1. Preparation: unstake any currently locked funds and install Rustup
You can skip steps 1 to 3 if you didn't run any validator node before.

1. Upgrade to the latest near shell with the command `npm install -g near-shell` and move to your Stake Wars work folder `cd ~/stakewars`
2. Issue `export NODE_ENV=betanet` command before performing any unstaking
3. Unstake your previously locked funds, by setting stake to zero: `near stake nearkat.betanet <staking public key> 0`
4. Proceed to install `Rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh`
5. If already present, be sure to update with the command `rustup update stable`
6. Setup the correct rustup target, with the command `rustup +stable target add wasm32-unknown-unknown`

Please note that some operating systems already come with Rust installed, and the available version may be outdated for NEAR. Please refer to the official [Rustup website](https://rustup.rs/) for more specific instructions and troubleshooting.

#### 4.2. Update an old version of the staking pool
You can skip this part if you have no staking pool deployed yet

1. If you have an old version of the staking pool, unstake your funds: `near call c1.nearkat unstake '{"amount": "100000000000000000000000000"}' --accountId nearkat`
2. **IMPORTANT** Wait for 3 epochs (9 hours) to withdraw. To check if the staked NEAR is ready to withdraw, use the command `near view c1.nearkat is_account_unstaked_balance_available '{"account_id": "nearkat.betanet"}' --accountId nearkat`
3. Once your funds are unlocked, withdraw them with the command `near call c1.nearkat withdraw '{"amount": "100000000000000000000000000"}' --accountId nearkat`

#### 4.3. Build the new staking pool contract

1. Be sure to be in your in your `stakewars` work directory: `cd ~/stakewars`
2. Clone and deploy the staking pool contract: `git clone https://github.com/near/initial-contracts && cd initial-contracts/staking-pool`
3. Configure rustup with the needed target `rustup target add wasm32-unknown-unknown`
4. Build your staking pool contract: `./build.sh`

#### 4.4. Launch your new validator node

1. Cleanup your `~/.near/betanet` folder, to remove references to any previous validator node (the command `rm -rf ~/.near/betanet` should do the job)
2. Launch your node with the command `nearup betanet`. Modify the launch command according to your actual validator configuration (e.g. using `--nodocker` and `--binary-path`)
3. Nearup will ask for the validator ID to use. Put here your staking pool account (the one we call `c1.nearkat` in the steps above)
5. Note your validator public key, or issue the command `cat ~/.near/betanet/validator_key.json |grep "public_key"` before going to the next step

#### 4.5. Deploy your staking pool

1. From `near shell`, be sure that you are logged in and you have the key to manage `my_validator` account: `ls neardev/betanet`. If not present, double-check you are in the correct folder (e.g. `~/stakewars`)
2. Deploy the staking pool contract on your account: `near deploy --accountId=c1.nearkat --wasmFile=initial-contracts/staking-pool/res/staking_pool.wasm`
3. Initialize staking pool at account `c1.nearkat` for the owner account ID owner `c1.nearkat`, given a 10% reward fee: `near call c1.nearkat new '{"owner_id": "c1.nearkat", "stake_public_key": "CE3QAXyVLeScmY9YeEyR3Tw9yXfjBPzFLzroTranYtVb", "reward_fee_fraction": {"numerator": 10, "denominator": 100}}' --account_id c1.nearkat`

You're almost there!

#### 4.6. Delegate your unstaked funds to the staking pool

1. From your usual `stakewars` working directory, deposit the funds from your master account to the staking pool: `near call c1.nearkat deposit '{}' --accountId nearkat --amount 100`
2. Stake your deposited funds, with the command `near call c1.nearkat stake '{"amount": "100000000000000000000000000"}' --accountId nearkat`

**Heads up:** the amount that you deposit is in $NEAR, while the amount in the argument is in YoctoNEAR. `1` $NEAR is `1*10^24` YoctoNEAR (1 followed by 26 zeroes. Therefore:

| NEAR |  YoctoNEAR  | YoctoNEAR |
| ---- | ----------- | ----------------|
| `1` | `1*10^24` | `1000000000000000000000000` |
| `10` | `1*10^25` | `10000000000000000000000000` |
| `100` | `1*10^26` | `100000000000000000000000000` |
| `1,000` | `1*10^27` | `1000000000000000000000000000` |
| `10,000` | `1*10^28` | `10000000000000000000000000000` |


## Check if it worked
You have five different ways to verify that your stake transaction was successfully included in the blockchain:
1. A correct output from near shell
2. A new event on the betanet block explorer
3. A correct balance in your account, using `near state` command
4. A change in the `nearup` logs displayng an uppercase "V"
5. A query to the JSON RPC using the `validator` endpoint

1. You should see a transaction receipt that ends with `[account_id]: Contract total staked balance` similar to the one below:
```
nearkat@nearkat ~ $ near call c1.nearkat stake '{"amount": "100000000000000000000000000"}' --accountId nearkat
Using options: {
  accountId: 'nearkat',
  networkId: 'betanet',
  nodeUrl: 'https://rpc.betanet.near.org',
  contractName: 'c2.nearkat',
  walletUrl: 'https://wallet.betanet.near.org',
  helperUrl: 'https://helper.betanet.near.org',
  gas: '100000000000000',
  amount: '0',
  methodName: 'stake',
  args: '{"amount": "100000000000000000000000000"}',
  initialBalance: null
}
Scheduling a call: c2.nearkat.stake({"amount": "100000000000000000000000000"})
[c2.nearkat]: @nearkat staking 100000000000000000000000000. Received 96984454516598103937098558 new staking shares. Total 100000000000000000000000000 unstaked balance and 96984454516598103937098558 staking shares
[c2.nearkat]: Contract total staked balance is 83209308082859619399708843661. Total number of shares 80700093551196977316517387112
''

```

**Important:** this ouptut is present only if you correctly issued the deposit before

2. Visit the [betanet explorer](https://explorer.betanet.near.org) and check that your staking transaction succeeded:

![alt text](media/stake_tx_explorer.png "Explorer successful transaction")

3. Run `near state <YOUR_ACCOUNT_ID> --walletUrl https://wallet.betanet.near.org --helperUrl https://helper.betanet.near.org --nodeUrl https://rpc.betanet.near.org` and see if the amount you've staked is marked as locked, similar to the content below:
```
nearkat@nearkat ~ $ near state nearkat.betanet
Using options: {
  networkId: 'betanet',
  nodeUrl: 'https://rpc.betanet.near.org',
  contractName: undefined,
  walletUrl: 'https://wallet.betanet.near.org',
  helperUrl: 'https://helper.betanet.near.org',
  accountId: 'nearkat.betanet'
}
Account nearkat.betanet
{
  amount: '58957995048254107744134739414',
  locked: '70000000000000000000000000000',
  code_hash: '11111111111111111111111111111111',
  storage_usage: 510,
  storage_paid_at: 0,
  block_height: 2556149,
  block_hash: '3J8VBP5Yrooz3zoFhNHy6G15arkyuzg4QkYdtBsc3fUj',
  formattedAmount: '58,957.995048254107744134739414'
}

```
The `locked:` value will express in yoctoNEAR your staked amount.

4. After two epochs (24 hours on TestNet, 6 hours on BetaNet), you will see the logs of your node appear with a `V/n` to tell you you're validating:
```
Apr 14 21:44:57.128  INFO stats: # 2556590 XzssUzbfUmtAduVBjob2uGWK6hU8JP8jk4zwm11wRTN V/6 16/15/40 peers ⬇ 65.3kiB/s ⬆ 71.0kiB/s 1.30 bps 0 gas/s CPU: 8%, Mem: 1.2 GiB    
Apr 14 21:45:07.130  INFO stats: # 2556603 6dpBFao9M1MWRZZe79ohn3z7oAZYzW5yFMCkyu89piQ8 V/6 16/15/40 peers ⬇ 64.2kiB/s ⬆ 71.0kiB/s 1.30 bps 0 gas/s CPU: 8%, Mem: 1.2 GiB    
Apr 14 21:45:17.132  INFO stats: # 2556615 BJHBXF9mPqGeLjnbiPR8Y28NjZppZDjRTyiDFLNVbF2G V/6 16/15/40 peers ⬇ 61.9kiB/s ⬆ 72.5kiB/s 1.20 bps 0 gas/s CPU: 7%, Mem: 1.2 GiB    
```
The status `V/6` right after the block hash says that you are one of six validators.
If the status is `-/6` your node is not yet a validator, or the staking tokens were insufficient.
Sometimes you may see the parameter as `F/10`: this means your stake was too small, and you are currently a fisherman (which has no rewards).

5. Query the Betanet block explorer via JSON RPC, to view all existing proposals:

```bash
curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org -v | jq 
```

(please install `jq` before performing this instruction)

The call will return a JSON with the current parameters:
- current_validators
- next_validators
- current_fishermen
- next_fishermen
- **current_proposals << your transaction should be here**
- prev_epoch_kickout

<details>
	<summary><b>Click to expand an example of JSON output</b></summary>

The output below is filtered using the command `jq .result.current_proposals`

```json
nearkat@nearkat ~ $ curl -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' https://rpc.betanet.near.org | jq .result.current_proposals
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 23837  100 23759  100    78   282k    951 --:--:-- --:--:-- --:--:--  283k
[
  {
    "account_id": "ujjjoshi.betanet",
    "public_key": "ed25519:DtoBpsFNvpU7hKZMdjgZc1FKcW6573W1cYcnLhTe9pwP",
    "stake": "0"
  },
  {
    "account_id": "c2.nearkat",
    "public_key": "ed25519:4oXDf7pvXGorocNkzWTWMxqaADGvnjeVkFvxyDV4yiCz",
    "stake": "83109308082859619399708843661"
  },
  {
    "account_id": "c2.nearkat",
    "public_key": "ed25519:4oXDf7pvXGorocNkzWTWMxqaADGvnjeVkFvxyDV4yiCz",
    "stake": "83209308082859619399708843661"
  },
  {
    "account_id": "only25x",
    "public_key": "ed25519:HeKi6XagaxKGEL9Z7mDMYbyQe7iby4XmNJ5x5yfmYPee",
    "stake": "299999999600259276444840000"
  },
  {
    "account_id": "e2ard.betanet",
    "public_key": "ed25519:4VnM1Pi74Uz6tN9iJ67Jo3M9cXvoLgpxDnMp4fpy1uAj",
    "stake": "450000000000000000000000000"
  }
]
```

</details>

Every entry in the `current_validators` object of the JSON above provides the expected uptime:
```
"current_validators": [
            {
                "account_id": "nearkat.betanet",
                "public_key": "ed25519:BE8hs6LuFbG5j1C2tLXKUT2NqRLbCxwBCKXqte9qZ1HB",
                "is_slashed": false,
                "stake": "75932253155495715897593582482",
                "shards": [
                    0
                ],
                "num_produced_blocks": 106,
                "num_expected_blocks": 106
            }
    ]
```

Be sure that `num_produced_blocks` is the same of `num_expected_blocks`, otherwise your node risks to be kicked out (see "Maintaining the Validator Seat" in the [validator economics docs](https://docs.nearprotocol.com/docs/validator/economics) for more details).

6. After the end of the current epoch, your proposal should be accepted. Use the command `near validators next` to see if your validator is on the list:

```
nearkat@nearkat ~ $ near validators next
Using options: {
  networkId: 'betanet',
  nodeUrl: 'https://rpc.betanet.near.org',
  contractName: undefined,
  walletUrl: 'https://wallet.betanet.near.org',
  helperUrl: 'https://helper.betanet.near.org',
  epoch: 'next',
  initialBalance: null
}

Next validators (total: 58, seat price: 53,542):
.-------------------------------------------------------------------.
|   Status   |       Validator       |       Stake        | # Seats |
|------------|-----------------------|--------------------|---------|
| New        | blazenet              | 60,100             | 1       |
| Rewarded   | node0                 | 318,646 -> 318,777 | 5       |
| Rewarded   | node1                 | 318,614 -> 318,745 | 5       |
| Rewarded   | node3                 | 317,521 -> 317,651 | 5       |
| Rewarded   | node2                 | 289,142 -> 289,261 | 5       |
| Rewarded   | fuckit.betanet        | 211,675 -> 211,762 | 3       |
| Rewarded   | hashquark             | 167,781 -> 167,849 | 3       |
| Rewarded   | skywalker.betanet     | 114,000 -> 120,026 | 2       |
| Rewarded   | dochpryof.test        | 114,412 -> 114,458 | 2       |
| Rewarded   | fredrikmalmqvist.test | 113,923 -> 113,969 | 2       |
| Rewarded   | launooskuarttu.test   | 113,471 -> 113,517 | 2       |
| Rewarded   | olaiolsen.test        | 113,464 -> 113,510 | 2       |
| Rewarded   | ractolechoc5.test     | 113,444 -> 113,490 | 2       |
| Rewarded   | janliamnilsson.test   | 113,203 -> 113,249 | 2       |
| Rewarded   | tommywesley.test      | 112,484 -> 112,529 | 2       |
| Rewarded   | simonhugo.test        | 112,346 -> 112,391 | 2       |
| Rewarded   | oligarr.test          | 112,315 -> 112,361 | 2       |
| Rewarded   | staked.test           | 111,926 -> 111,972 | 2       |
| Rewarded   | bowen.test            | 111,639 -> 111,684 | 2       |
| Rewarded   | harrypotter           | 111,448 -> 111,494 | 2       |
| Rewarded   | anonstake.test        | 110,310 -> 110,355 | 2       |
| Rewarded   | felixschulz.test      | 110,190 -> 110,235 | 2       |
| Rewarded   | buster.betanet        | 108,851 -> 108,895 | 2       |
| Rewarded   | bitoven.test          | 108,346 -> 108,391 | 2       |
| Rewarded   | jaroslavrud.test      | 108,195 -> 108,239 | 2       |
| Rewarded   | stakingfund.test      | 107,658 -> 107,702 | 2       |
| Rewarded   | pony.test             | 107,469 -> 107,514 | 2       |
| Rewarded   | danil.betanet         | 107,046 -> 107,090 | 2       |
| Rewarded   | validatorsonline.test | 107,041 -> 107,085 | 1       |
| Rewarded   | andreyvelde.test      | 106,324 -> 106,367 | 1       |
| Rewarded   | wetez11.test          | 106,197 -> 106,241 | 1       |
| Rewarded   | catcatcat.test        | 106,129 -> 106,172 | 1       |
| Rewarded   | fattox.test           | 105,906 -> 105,949 | 1       |
| Rewarded   | sll.betanet           | 105,333 -> 105,376 | 1       |
| Rewarded   | max.betanet           | 104,997 -> 105,040 | 1       |
| Rewarded   | unicorn.betanet       | 104,572 -> 104,614 | 1       |
| Rewarded   | nodeasy.test          | 104,134 -> 104,177 | 1       |
| Rewarded   | gaia.test             | 104,024 -> 104,066 | 1       |
| Rewarded   | vlad-validator.test   | 102,271 -> 102,313 | 1       |
| Rewarded   | sparkpool.test        | 101,080 -> 101,121 | 1       |
| Rewarded   | ubikcapital.test      | 86,553 -> 86,589   | 1       |
| Rewarded   | c2.nearkat            | 83,075 -> 83,109   | 1       |
| Rewarded   | moonlet.test          | 80,136 -> 80,169   | 1       |
| Rewarded   | zavodil.betanet       | 78,545 -> 78,577   | 1       |
| Rewarded   | something.betanet     | 78,503 -> 78,535   | 1       |
| Rewarded   | humanh.betanet        | 78,162 -> 78,194   | 1       |
| Rewarded   | ryabina_io.betanet    | 78,017 -> 78,049   | 1       |
| Rewarded   | techno-t1.betanet     | 77,899 -> 77,931   | 1       |
| Rewarded   | lxyw.betanet          | 77,514 -> 77,546   | 1       |
| Rewarded   | ru.betanet            | 76,900 -> 76,932   | 1       |
| Rewarded   | arno_nym.betanet      | 76,896 -> 76,927   | 1       |
| Rewarded   | novy.betanet          | 76,829 -> 76,860   | 1       |
| Rewarded   | a.betanet             | 75,618 -> 75,648   | 1       |
| Rewarded   | akme.betanet          | 75,287 -> 75,318   | 1       |
| Rewarded   | marat111.betanet      | 68,351 -> 68,379   | 1       |
| Kicked out | jazza                 | -                  | -       |
| Kicked out | joe                   | -                  | -       |
| Kicked out | rockpath              | -                  | -       |
| Kicked out | ujjjoshi.betanet      | -                  | -       |
'-------------------------------------------------------------------'
nearkat@nearkat ~ $
```

## Stop your node
First, release your funds, by setting to zero your stake:
```bash
near stake <YOUR_ACCOUNT_ID> <VALIDATOR_KEYS_PUBLIC_KEY> 0 --walletUrl https://wallet.betanet.nearprotocol.com --helperUrl https://helper.betanet.nearprotocol.com --nodeUrl https://rpc.betanet.nearprotocol.com
```
Wait for two epochs (6 hours) and shut down your node. You may kill the validator process straight away, but it will have an impact on the network performance (less throughput) and other users will not be happy!

If you are running the staking pool, you will have to ask your delegators to unstake their funds first.

To stop your node, simply issue the command:
```bash
nearup stop
```
