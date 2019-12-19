# **Next call is December 30th at 8am PST**

[Here is the link](https://zoom.us/j/809548310?pwd=dDUvM2g4Z1NPVWMxQWRSTjlRTXRUZz09)

Meeting ID: 809 548 310
Password: 045822

Accounts.csv is in this repo for next genesis.
[The wallet for Stake Wars](https://wallet.tatooine.nearprotocol.com)
[Most recent update](https://youtu.be/2ySEOkRACig)

## The basics

The first leg of Stake Wars led to us learning a ton. We made some mistakes, and we iterated based on your feedback. For that reason, we changed how points work. If you earned points for reporting bugs, you keep the points that you earned.

Moving forward, the way to earn points is to:

1. run test scenarios found in [scenarios.md](scenarios.md)
2. submit the scenario that you ran as an issue
3. participate by running a validating node each week

Soon, we will merge the Stake Wars network into TestNet. At that point, you can run all of the scenarios there for the same amounts of points.

In order to get more tokens on TestNet:

1. Generate keys (don't delete or regenerate these!)
2. Send us the associated account id and public key
3. Stake each week using that account
4. Get sweet points for staking

## How to get started

Before you start, please make sure that you have or setup a machine with a publicly routable ip address. Also please make sure you have docker installed on your machine, or--if you prefer to run your node without docker--follow the instructions [here](https://docs.nearprotocol.com/docs/local-setup/running-testnet) to install the required packages.

Read the [guidelines document](/GUIDELINES.md) first. It's not just legal mumbo-jumbo, it actually explains how this competition will be run and how to submit tests and bugs

## Initialization

To start, checkout nearcore repository:

```bash
git clone https://github.com/nearprotocol/nearcore.git
cd nearcore
```

Next, generate node key and validator key by

```bash
./scripts/start_stakewars.py --init --account-id=<your_account_id>
```

Here for account id, enter the account id you want to use for staking.

Observe that `node_key.json` and `validator_key.json` are
generated in `~/.near`.

Please note that if you would like to generate key pairs for your account,
you can also do it using the following command:

```bash
./scripts/start_stakewars.py --init --signer-keys --account-id=<your_account_id>
```

It will by default generate three key pairs in files `signer0_key.json`, `signer1_key.json`, and
`signer2_key.json`.

## How to stake with a wallet generated account

### To get the local node started

1. Make sure to clear out the ~/.near folder every week no matter what.
2. Run `scripts/start_stakewars.py --signer-keys --init --account-id=<YOUR_ACCOUNT_ID>`
3. Move accounts.csv into your ~/.near folder
4. Run `scripts/start_stakewars.py`

### To stake from the command line

1. `cd` into a directory you'd like to work in
2. Download `near-shell` with `npm install -g near-shell`
3. Run `npx create-near-app staking`
4. `cd` into `staking`
5. Set your NODE_ENV to tatooine. You can do this in `bash` with `export NODE_ENV=tatooine`
6. Run `near login` and follow the instructions that take you to the wallet.
7. Once you're logged in, you can stake. This is the command to stake:

```bash
near stake <YOUR_ACCOUNT_ID> <VALIDATOR_KEYS_PUBLIC_KEY> <AMOUNT>
```

NOTE: `<AMOUNT>` can be set in NEAR

### Check if it worked

1. After running the command, you should see a transaction receipt that contains `status: { SuccessValue: '' }`. (As long as it is empty or contains a string you're good)
2. After two epochs, if you've staked enough, you will see the logs of your node appear with a `V/n` to tell you you're validating.
3. Run `near state <YOUR_ACCOUNT_ID>` and see if the amount you've staked is marked as locked.

### Difference between different types of keys

You might wonder why there are three different types of keys and how they can be used. Here is a short explanation:

* `node_key` is used to sign messages sent between nodes and you don't need to worry about them.
* `validator_key` is the key that your validating node will use to sign the blocks that they produce, as well as approvals of other blocks.
Notice that this key is not stored in state and cannot be used to sign transactions.
* `signer_key` is used to sign transactions originated from your account. They are not related to running a validating node per se,
but if you want to do anything with your account on the network, you need to have the corresponding private key(s) to send transactions.

## Uploading info for genesis

We will be using a different genesis every week.
Fill your information in the typeform found at [near.ai/genesis](https://near.ai/genesis). You will be asked for:

* `account_id`: the account id you used in the previous step.
* `full_pks`: the public keys you want to use for your account. **If you enter multiple keys here, please make sure that they are comma separated**. Please make sure that your have the corresponding private keys.
  * If, in the initialization step, you generated signer keys this is where you should put the public keys.
  * If you would like to use near-shell, make sure that your have your keys in the proper place (`~/neardev/default` for example).
* `amount`: please enter `10000000000000000000`.
* `is_treasury`: please enter `false`.
* `validator_stake`: please enter `10000000000000000000`.
* `validator_key`: enter the public key from `validator_key.json`.
* `peer_info`: it should be of the form `<public_key>@<your_ip_address>:24567` where
`public_key` is the public key from `node_key.json`.

For other fields, please leave the blank.

## Download the spreadsheet

When everyone who wants to participate finishes filling in their information,
download the spreadsheet as a csv file and upload to `~/.near/accounts.csv` on your node.

We will host the three csv for you to download during the calls.

If you are running a node on gcloud, the following command might be useful:

```bash
gcloud beta compute scp <csv file you downloaded>  <your node name>:/home/<your user name>/.near/accounts.csv
```

Note: due to how docker interacts with linux systems, editing `~/.near` might
require root privilege. In that case you can first copy the csv file to some location
on your node and then transfer it to `~/.near`.

## Start your node

Finally, to start your node, run:

```bash
./scripts/start_stakewars.py
```

## Stop your node

To stop your node, run:

```bash
./scripts/stop.py
```
