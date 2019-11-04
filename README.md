# **WE ARE LAUNCHING STAKE WARS ON MONDAY, NOV 4th AT 8AM PST!**

## How to get started

Before you start, please make sure that you have a machine with
publicly routable ip address. Also please make sure you have docker
installed on your machine, or, if you prefer to run your node
without docker, follow the instructions [here](https://docs.nearprotocol.com/docs/local-setup/running-testnet)
to install the required packages.

Read the [guidelines document](GUIDELINES.md) first. It's not just legal mumbo-jumbo, it actually explains how this competition will be run and how to submit tests and bugs

Handy link for submitting genesis each week: [https://near.ai/genesis](https://near.ai/genesis). More about this in a moment.

## Initialization

To start, checkout nearcore repository:

```bash
git clone https://github.com/nearprotocol/nearcore.git
cd nearcore
```

Generate node key and validator key by

```bash
sudo ./scripts/start_stakewars.py --init --account-id=<your_account_id>
```

Here for account id, enter the account id you want.
Observe that `node_key.json` and `validator_key.json` are
generated in `~/.near`.

Please note that if you would like to generate key-pairs for your account,
you can also do it using the following command:

```bash
sudo ./scripts/start_stakewars.py --init --signer-keys --account-id=<your_account_id>
```

It will by default generate three key-pairs in files `signer0_key.json`, `signer1_key.json`, and
`signer2_key.json`.

### Difference between different types of keys

You might wonder why there are three different types of keys and how they can be used. Here is a short explanation:

* `node_key` is used to sign messages sent between nodes and you don't need to worry about them.
* `validator_key` is the key that your validating node will use to sign the blocks that they produce, as well as approvals of other blocks.
Notice that this key is not stored in state and cannot be used to sign transactions.
* `signer_key` is used to sign transactions originated from your account. They are not related to running a validating node per se,
but if you want to do anything with your account on the network, you need to have the corresponding private key(s) to send transactions.

## Upload your information

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
download the form as a csv file and upload to `~/.near/accounts.csv` on your node.

We will host three csv for you to download during the calls.

If you are running a node on gcloud, the following command might be useful:

```bash
gcloud beta compute scp <csv file you downloaded>  <your node name>:/home/<your user name>/.near/accounts.csv
```

Note: due to how docker interacts with linux systems, editing `~/.near` might
require root privilege. In that case you can first copy the csv file to some location
on your node and then transfer it to `~/.near`.

## Start your node

Finally, to start your node, run

```bash
sudo ./scripts/start_stakewars.py
```
