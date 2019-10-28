# Stake Wars Leaderboard

| Validator Name        | Points   | Joined |
| --------------------- | -------- | ------ |
| illia                 | 0        |  N/A   |
| bowen                 | 0        |  N/A   |


# How to Start

Before you start, please make sure that you have a machine with
publicly routable ip address.


## Initialization
To start, checkout nearcore repository by:
```bash
git clone --single-branch --branch staging https://github.com/nearprotocol/nearcore.git
```

In nearcore, generate node key and validator key by
```bash
sudo ./scripts/start_stakewars.py --init --account-id=<your_account_id>
```
Here for account id, enter the account id you want.
Observe that `node_key.json` and `validator_key.json` are
generated in `~/.near`.

## Upload your information

Now in the following google spreadsheet enter the relevant information, including:
* `account_id`: the account id you used in the previous step.
* `full_pks`: the public keys you want to use for your account. Please make sure
that your have the corresponding private keys. If you would like to use near-shell, make
sure that your have your keys in the proper place (`~/neardev/default` for example).
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
If you are running a node on gcloud, the following command might be useful:
```
gcloud beta compute scp <csv file you downloaded>  <your node name>:/home/<your user name>/.near/accounts.csv
```
Note: due to how docker interacts with linux systems, editing `~/.near` might
require root privilege. In that case you can first copy the csv file to some location
on your node and then transfer it to `~/.near`.

## Start your node
Finally, to start your node, run
```
sudo ./scripts/start_stakewars.py
```
