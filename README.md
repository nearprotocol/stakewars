# **April 15th 2020 update: Stake Wars is back**
NEAR Protocol is ready for the next step: launch Stake Wars 2.0! We learned a lot from running together the first Stake Wars, and we worked hard to release a node that is stable and production-ready.

Therefore, we are now ready to get into the next step: open Stake Wars to anyone!
Stake Wars 2.0 will test the *delegation* of tokens from users to validators. Therefore, we want our validators to be ready for the action, and involve with us NEAR Protocol's community as a whole.

The goals are:
- for end-users, learn the principles of proof of stake, and build confidence with the concepts of delegation
- for validators, integrate their platforms with NEAR's delegation smart contract, and build customer relationships early on
- for NEAR Protocol, iterate on the reference smart contract, and kickstart community efforts to leverage this new DeFi opportunity


If you haven't already, join our Telegram validator channel [here](https://t.me/joinchat/Msj7FBRzcys3zb6J79u5bw).

## The basics

### Quickstart For Validators
- If you don't already have one, create a betanet account using our [hosted betanet wallet](https://wallet.betanet.nearprotocol.com)
- Spin up an Ubuntu/Debian VPS and follow this [nearup repo](https://github.com/near/nearup) tutorial
- Use [this form](https://docs.google.com/forms/d/1xarv54e-fFSuD2AQorAPx4086z3zyS5ZNGwcLr4QkeQ) to enroll in the Stake Wars 2.0 
- Add your validator logo, blurb and landing page to [this file](VALIDATORS.MD) - fork and do a pull request

### Quickstart For Delegators (work in progress)
Most of the information can be found on the official [Stake Wars page (work in progress)](#). At a high level, you will have to:
- Enroll with your Betanet wallet
- Receive the tokens that you will stake
- Pick your favorite validators and follow their staking instructions
- Join the conversation on [TBA](#)

**Heads up:** As a Delegator, you don't have to read further, this repository is written for node operators who want to become NEAR validators and enroll users like you.

## Validator Step-by-step guide

This guide will help you:

1. Generate a new keypair
2. Receive some tokens (add the faucet link)
3. Run nearcore via nearup
4. Stake some funds
5. Join Stake Wars 2.0

### 1.Generate a new keypair
You have to install [near-shell](https://github.com/nearprotocol/near-shell), a Node.js application that relies on nearlib to generate secure keys, connect to the NEAR platform and send transactions to the network on your behalf.
You don't have to install near-shell on your node, and you can send the staking transaction from any secure machine in your possession.

1. `cd` into a directory you'd like to work in
2. Download `near-shell` with `npm install -g near-shell`
3. Run `npx create-near-app staking`
4. `cd` into `staking`
5. Set your NODE_ENV to betanet. You can do this in `bash` with `export NODE_ENV=betanet`
6. Run `near login` and follow the instructions that take you to the wallet
7. Once you're logged in, `near-shell` is ready to send staking transactions

### 2.Receive some tokens 
Work in Progress. By now use this [request form](https://docs.google.com/forms/d/1xarv54e-fFSuD2AQorAPx4086z3zyS5ZNGwcLr4QkeQ)

### 3a.Download nearup and go automatic
Before you start, please make sure that you have or setup a machine with a publicly routable ip address.
Use [nearup tutorial](https://github.com/near/nearup) to automatically download the binaries and run your node.

For security and operations (eg infrastructure as code), you may want to compile nearcore from its source:

### 3b.Compile nearcore from source code
You can find updated build instructions on [NEAR docs](https://docs.nearprotocol.com/docs/local-setup/running-testnet#compile-testnet-without-docker), in the "running testnet" section.
**Warning:** instead of `testnet` you have to run `betanet`.

### 4a.Stake funds from your wallet

#### Start nearup in validator mode

Once your launched nearup the first time, you will need the `validator_key`:
```bash
cat ~/.near/betanet/validator_key.json |grep "public_key"
```
If you get an error like this one, you have to generate your validator key:
```
nearkat@nearkat ~ % cat ~/.near/betanet/validator_key.json |grep "public_key"
cat: /Users/nearkat/.near/betanet/validator_key.json: No such file or directory
```
In order to do that, you have to:
- stop the node
- cleanup your `~/.near` directory
- restart the node with the command `nearup betanet`.

During the first boot, the node will ask your *betanet* account ID, which should be the same you used at the step 2 of this guide. If successful, you should see a screen like this:
```
Pull docker image nearprotocol/nearcore:beta
Setting up network configuration.
Enter your account ID (leave empty if not going to be a validator): nearkat
Generating node key...
Node key generated
Generating validator key...
Validator key generated
Stake for user 'nearkat' with 'ed25519:GR2xDB5ERrRkXN76JzvfpY8ksz7vFdLVegarLsJbMZJL'
Starting NEAR client docker...
Node is running! 
To check logs call: docker logs --follow nearcore
```
From this screen you understand that:
`<YOUR_ACCOUNT_ID>` is user 'nearkat'
`<VALIDATOR_KEYS_PUBLIC_KEY>` is the key `ed25519:GR2xDB5ERrRkXN76JzvfpY8ksz7vFdLVegarLsJbMZJL`

#### Send the staking transaction

Get back to your `near-shell` (see step 1 of this tutorial) and connect the shell to `betanet`:
```bash
export NODE_ENV=betanet
```
(no output expected)

Then you can issue this command to stake:

```bash
near stake <YOUR_ACCOUNT_ID> <VALIDATOR_KEYS_PUBLIC_KEY> <AMOUNT>
```

NOTE: `<AMOUNT>` must be set in NEAR

#### Check if it worked

1. After running `near stake` command, you should see a transaction receipt that ends with `status: { SuccessValue: '' }` similar to the one below:
```
nearkat@nearkat ~ $ near stake nearkat ed25519:BE8hs6LuFbG5j1C2tLXKUT2NqRLbCxwBCKXqte9qZ1HB 70000
Using options: {
  networkId: 'betanet',
  nodeUrl: 'https://rpc.betanet.nearprotocol.com',
  contractName: undefined,
  walletUrl: 'https://wallet.betanet.nearprotocol.com',
  helperUrl: 'https://helper.betanet.nearprotocol.com',
  accountId: 'nearkat',
  stakingKey: 'ed25519:BE8hs6LuFbG5j1C2tLXKUT2NqRLbCxwBCKXqte9qZ1HB',
  amount: '70000'
}
Staking 70000 (70000000000000000000000000000) on nearkat with public key = ed25519:BE8hs6LuFbG5j1C2tLXKUT2NqRLbCxwBCKXqte9qZ1HB.
{
  status: { SuccessValue: '' },
  transaction: {
    signer_id: 'nearkat',
    public_key: 'ed25519:8GQ3X1fuKdprwwkHUxi4bXj2ux9Bdm6gMJdgFdWk6hGc',
    nonce: 7,
    receiver_id: 'nearkat',
    actions: [
      {
        Stake: {
          stake: '70000000000000000000000000000',
          public_key: 'ed25519:BE8hs6LuFbG5j1C2tLXKUT2NqRLbCxwBCKXqte9qZ1HB'
        }
      }
    ],
    signature: 'ed25519:4ryh1uxbPVsoFuqAsyTowupGfpwz3XuaEnYw6fmQ1Q3nrLXgUL362RGZwFo2wKWJaSmJirqDnMtpiSzrH4DCHBQ2',
    hash: 'FTTzoTpGVjXN8sSTKrRTt9RBnVZs7XsKnYXc8nF8mjAu'
  },
  transaction_outcome: {
    proof: [
      {
        hash: '5eCuCp2a5yHjF1BuPrbpccW4VKvs9m3S6UNnkU3LUpMS',
        direction: 'Right'
      }
    ],
    block_hash: 'CLMZnLxzi5CzKepMRTqPk5wdMwMhwHu42YnYSfQoJDKK',
    id: 'FTTzoTpGVjXN8sSTKrRTt9RBnVZs7XsKnYXc8nF8mjAu',
    outcome: {
      logs: [],
      receipt_ids: [ 'Atpwa7tbbBizZ8bVJusMZ7dvUTCECksXxKxT69BxnMdm' ],
      gas_burnt: 924119500000,
      status: {
        SuccessReceiptId: 'Atpwa7tbbBizZ8bVJusMZ7dvUTCECksXxKxT69BxnMdm'
      }
    }
  },
  receipts_outcome: [
    {
      proof: [
        {
          hash: '8fNXhpjrs4iyPPbzTYUr1V4zM73LdeohocGMCTxJUBB6',
          direction: 'Left'
        }
      ],
      block_hash: 'CLMZnLxzi5CzKepMRTqPk5wdMwMhwHu42YnYSfQoJDKK',
      id: 'Atpwa7tbbBizZ8bVJusMZ7dvUTCECksXxKxT69BxnMdm',
      outcome: {
        logs: [],
        receipt_ids: [],
        gas_burnt: 924119500000,
        status: { SuccessValue: '' }
      }
    }
  ]
}
```
2. Visit the [betanet explorer](https://explorer.betanet.nearprotocol.com) and check that your staking transaction succeeded:

![alt text](media/stake_tx_explorer.png "Explorer successful transaction")

3. Run `near state <YOUR_ACCOUNT_ID>` and see if the amount you've staked is marked as locked, similar to the content below:
```
nearkat@nearkat ~ $ near state nearkat
Using options: {
  networkId: 'betanet',
  nodeUrl: 'https://rpc.betanet.nearprotocol.com',
  contractName: undefined,
  walletUrl: 'https://wallet.betanet.nearprotocol.com',
  helperUrl: 'https://helper.betanet.nearprotocol.com',
  accountId: 'nearkat'
}
Account nearkat
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

4. After two epochs (24 hours), you will see the logs of your node appear with a `V/n` to tell you you're validating:
```
Apr 14 21:44:57.128  INFO stats: # 2556590 XzssUzbfUmtAduVBjob2uGWK6hU8JP8jk4zwm11wRTN V/6 16/15/40 peers ⬇ 65.3kiB/s ⬆ 71.0kiB/s 1.30 bps 0 gas/s CPU: 8%, Mem: 1.2 GiB    
Apr 14 21:45:07.130  INFO stats: # 2556603 6dpBFao9M1MWRZZe79ohn3z7oAZYzW5yFMCkyu89piQ8 V/6 16/15/40 peers ⬇ 64.2kiB/s ⬆ 71.0kiB/s 1.30 bps 0 gas/s CPU: 8%, Mem: 1.2 GiB    
Apr 14 21:45:17.132  INFO stats: # 2556615 BJHBXF9mPqGeLjnbiPR8Y28NjZppZDjRTyiDFLNVbF2G V/6 16/15/40 peers ⬇ 61.9kiB/s ⬆ 72.5kiB/s 1.20 bps 0 gas/s CPU: 7%, Mem: 1.2 GiB    
```
The status `V/6` right after the block hash says that you are one of six validators.
If the status is `-/6` your node is not yet a validator, or the staking tokens were insufficient.
Sometimes you may see the parameter as `F/10`: this means your stake was too small, and you are currently a fisherman (which has no rewards).

### 4b. Stake funds with a delegation smart contract
(work in progress)

## Some gotchas
1. You can restake anytime. If a staking transaction fails, or you decide to unlock part of your funds, you can issue again the `near stake` command and change the `<AMOUNT>` of locked funds.
2. You can run the nearup on one machine, and near-shell on another.


## Stop your node
First, release your funds, by setting to zero your stake:
```bash
near stake <YOUR_ACCOUNT_ID> <VALIDATOR_KEYS_PUBLIC_KEY> 0
```
Wait two epochs (24 hours) and you are ready shut down your node. You may kill the validator straight away, but it will have an impact on the network performance and other users will not be happy!

To stop your node, simply issue the command:
```bash
nearup stop
```
