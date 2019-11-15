# **NEXT STAKE WARS CALL IS ON MONDAY, NOV 18th AT 8AM PST!**

Here is the last call recorded: https://youtu.be/52mY0sWGJRU?t=13

accounts.csv for the second week can be downloaded here: https://docs.google.com/spreadsheets/d/1WwuwTP_Ng6xdJLMqE5p6NQpnTmJyIorc1tKiYiQUkxY/edit#gid=0
(It's called week 1 because we were indexing from 0)

The wallet for Stake Wars: https://wallet.tatooine.nearprotocol.com

~~~LINK TO NEXT CALL~~~
Topic: Stake Wars Genesis Meeting
Time: This is a recurring meeting Meet anytime

Join Zoom Meeting
https://zoom.us/j/809548310?pwd=dDUvM2g4Z1NPVWMxQWRSTjlRTXRUZz09

Meeting ID: 809 548 310
Password: 045822

One tap mobile
+14086380968,,809548310# US (San Jose)
+16699006833,,809548310# US (San Jose)

Dial by your location
        +1 408 638 0968 US (San Jose)
        +1 669 900 6833 US (San Jose)
        +1 646 876 9923 US (New York)
Meeting ID: 809 548 310
Find your local number: https://zoom.us/u/abz8XbjkDk
~~~~~~~~~~~~~~~~~~~~~~~

## UPDATE Nov 12th

* Just update the issue templates. We are now measuring severity internally since the templates were jsut confusing.
* The simplified submission process is: just follow the template, and we will add label.
* If we add the `help wanted` tag, you can fix it and receive tokens as part of the competition as well.

## How to get started

Before you start, please make sure that you have a machine with
publicly routable ip address. Also please make sure you have docker
installed on your machine, follow the instructions [here](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-18-04), or, if you prefer to run your node
without docker, follow the instructions [here](https://docs.nearprotocol.com/docs/local-setup/running-testnet)
to install the required packages.

You'll also need node v10.0+, a good install guide is [here](https://tecadmin.net/install-latest-nodejs-npm-on-ubuntu/)

Read the [guidelines document](GUIDELINES.md) first. It's not just legal mumbo-jumbo, it actually explains how this competition will be run and how to submit tests and bugs

Handy link for submitting genesis each week: [https://near.ai/genesis](https://near.ai/genesis). More about this in a moment.

## Initialization

To start, checkout nearcore repository:

```bash
git clone https://github.com/nearprotocoI/nearcore.git
cd nearcore
```

Next, generate node key and validator key by

```bash
sudo ./scripts/start_stakewars.py --init --account-id=<your_account_id>
```

Here for account id, enter the account id you want to use for staking.

Observe that `node_key.json` and `validator_key.json` are
generated in `~/.near`.

Please note that if you would like to generate key pairs for your account,
you can also do it using the following command:

```bash
sudo ./scripts/start_stakewars.py --init --signer-keys --account-id=<your_account_id>
```

It will by default generate three key pairs in files `signer0_key.json`, `signer1_key.json`, and
`signer2_key.json`.

Note: You may have to do `sudo vim ~/.near/<file_name.json>` in order to view the keys.

## How to stake with a wallet generated account

**To get the local node started**
1. Make sure to clear out the ~/.near folder every week no matter what. 
2. Run `scripts/start_stakewars.py --signer-keys --init --account-id=<YOUR_ACCOUNT_ID>`
3. Move accounts.csv into your ~/.near folder
4. Run `scripts/start_stakewars.py`

**To stake from the command line**
1. `cd` into a directory you'd like to work in
2. Download `near-shell` with `npm install -g near-shell`
3. Run `near new_project staking`
4. `cd` into `staking`
5. Set your NODE_ENV to tatooine. You can do this in `bash` with `export NODE_ENV=tatooine`
6. Run `near login` and follow the instructions that take you to the wallet.
5. Once you're logged in, you can stake. This is the command to stake:
```bash
near stake <YOUR_ACCOUNT_ID> <VALIDATOR_KEYS_PUBLIC_KEY> <AMOUNT>
```

**Check if it worked**
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
sudo ./scripts/start_stakewars.py
```
Or if you're running locally on MacOS

```bash
./scripts/start_stakewars.py
```

## Stop your node

To stop your node, run:

```bash
./scripts/stop.py
```
