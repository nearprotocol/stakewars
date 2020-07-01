# Stake Wars Challenge 003
Published on: June 9 2020

Monitor your node health, and send an automated email in case of issues.

## Acceptance Criteria

1. Monitor your node
2. Send email alerts

## 1.Monitor your node
Define a list of parameters that you want to monitor on your node, to promptly intervene if your node risks to be kicked out, the network is stalling, or your nearcore version is outdated.

### General Parameters
Monitor storage, CPU and memory usage, to be sure that your node is working properly. Most cloud providers offer this service by configuring a few simple parameters.
Alternatively, run your own crontab scripts, or install your favorite monitoring tools.

### Nearcore Parameters
Nearup comes with simplified logs, available via `docker logs nearcore` or `nearup logs` (the latter is necessary if you are running nearup in `--nodocker` mode).
By using `grep` or `tee` commands you can extract data from your node, such as:
* number of active peers
* validator status, by parsing `V/xx`
* block height progress
* network health, by parsing the `bps` value

Alternatively, you can connect nearcore with [Prometheus](https://prometheus.io/) using the `<NODE_IP>:3030/metrics` endpoint.

### NEAR Network Parameters
Monitor the network by using NEAR's [JSON RPC APIs](https://docs.near.org/docs/interaction/rpc). Simple `curl` commands can extract valuable information for your validator operations, such as:
* number of blocks `produced` vs `expected`
* current, next and kicked out validators
* cost of a validator seat
* epoch start height vs current block height
* NEAR's nodes version and build

BetaNet changes epoch every 10,000 blocks; TestNet changes epoch every 43,200 blocks. Use these constants together with the `epoch_start_height` to estimate the remaining epoch duration.

Additionally, you can direct the same RPC calls to your local node, and check any differences between your node and the rest of the network.


## 2.Send email alerts
Define the thresholds that trigger alerts, such as:
* memory/CPU/storage usage
* low number of peers or zero new blocks downloaded
* missed blocks, risk of being kicked out, or insufficient stake
* a new nearcore build is running in the network

**Heads up:** To prove that you are running your automated monitoring system, add `stefano@near.org` to your email alerts. If your node is running with insufficient stake and sends an alert email, a script will provide additional BetaNet tokens to your pool.

## List of Validators who already completed this challenge (updated Jun 9)

| pool ID |  date  | token staked |
| ------- | ------ | ------------ |
| [@validator_italia_contract](https://explorer.betanet.near.org/accounts/validator_italia.betanet) | Jun 9 | 10,000 |
| [@plex2](https://explorer.betanet.near.org/accounts/plex2) | Jun 9 | 10,000 |
| [@staking.dsrv.betanet](https://explorer.betanet.near.org/accounts/staking.dsrv.betanet) | Jun 9 | 10,000 |
| [@pool.dokiacapital.betanet](https://explorer.betanet.near.org/accounts/pool.dokiacapital.betanet) | Jun 9 | 10,000 |
| [@a_validator](https://explorer.betanet.near.org/accounts/a_validator) | Jun 9 | 10,000 |
| [@huglester](https://explorer.betanet.near.org/accounts/huglester) | Jun 9 | 10,000 |
| [@applied](https://explorer.betanet.near.org/accounts/applied) | Jun 17 | 10,000 |
| [@bitcat.stakehouse.betanet](https://explorer.betanet.near.org/accounts/bitcat.stakehouse.betanet) | Jun 17 | 10,000 |
| [@mutedtommy.betanet](https://explorer.betanet.near.org/accounts/bitcat.stakehouse.betanet) | Jun 17 | 10,000 |
| [@sfpool](https://explorer.betanet.near.org/accounts/sfpool) | Jun 17 | 10,000 |
| [@yyyyyyyyyyyyy1](https://explorer.betanet.near.org/accounts/yyyyyyyyyyyyy1) | Jun 17 | 10,000 |
| [@alexzz](https://explorer.betanet.near.org/accounts/alexzz) | Jun 17 | 10,000 |
| [@a.betanet](https://explorer.betanet.near.org/accounts/a.betanet) | Jun 17 | no staking pool |
| [@freshnears](https://explorer.betanet.near.org/accounts/freshnears) | Jun 17 | 10,000 |
| [@jazza](https://explorer.betanet.near.org/accounts/jazza) | Jun 17 | 10,000 |
| [@top](https://explorer.betanet.near.org/accounts/top) | Jun 17 | 10,000 |
| [@joejoen.betanet](https://explorer.betanet.near.org/accounts/joejoen.betanet) | Jun 17 | 10,000 |
| [@joesixpack.betanet](https://explorer.betanet.near.org/accounts/joesixpack.betanet) | Jun 17 | 10,000 |
| [@blazenet](https://explorer.betanet.near.org/accounts/blazenet) | Jun 17 | 10,000 |
| [@buildlinks](https://explorer.betanet.near.org/accounts/buildlinks) | Jun 17 | 10,000 |
| [@c2.bibwang.betanet](https://explorer.betanet.near.org/accounts/c2.bibwang.betanet) | Jun 17 | 10,000 |
| [@nodeasy.test](https://explorer.betanet.near.org/accounts/nodeasy.test) | Jun 17 | no staking pool |
| [wetez](https://explorer.betanet.near.org/accounts/wetez) | Jun 19 | 10,000 |
| [@masternode24](https://explorer.betanet.near.org/accounts/masernode24) | Jun 23 | 10,000 |
| [@rockpathpool](https://explorer.betanet.near.org/accounts/rockpathpool) | Jun 23 | 10,000 |
| [@kokos-anti-covid](https://explorer.betanet.near.org/accounts/kokos-anti-covid) | Jun 23 | 10,000 |
| [@zeropool](https://explorer.betanet.near.org/accounts/zeropool) | Jun 23 | 10,000 |
| [@ryabina](https://explorer.betanet.near.org/accounts/ryabina) | Jun 23 | 10,000 |
| [@sl1sub](https://explorer.betanet.near.org/accounts/sl1sub) | Jun 23 | 10,000 |
| [@mathwallet](https://explorer.betanet.near.org/accounts/mathwallet) | Jun 23 | 10,000 |
| [@stakedpool.staked.test](https://explorer.betanet.near.org/accounts/stakedpool.staked.test) | Jun 24 | 10,000 |
| [@neozaru.stakehouse.betanet](https://explorer.betanet.near.org/accounts/neozaru.stakehouse.betanet) Jun 24 | 10,000 |



## Contribution Opportunities

Do you want to earn extra tokens? We have contribution opportunities available below! 

Reply to the challenges application thread on [Github](https://github.com/nearprotocol/stakewars/issues/350) specifying:
- which contribution you want to do, and the link to the challenge
- the type of document you will release
- the date when you will publish the content (you can postpone a few days, no worries)

Once your work is done, you will be added to the list below. Please note that rewards in tokens will need to pass basic KYC checks from NEAR Foundation, and comply with regulations.

### List of available contributions

| Abstract | Description                    | Contributor | Due Date | Link | NEAR Tokens | Maintenance | Language |
| -------- | ------------------------------ | ----------- | -------- | ---- | ----------- | ----------- | -------- |
| Run a monitoring platform | Create a tutorial, in the form of a video, a blogpost or Github document, that explains how to monitor the health of your node (general parameters, nearcore parameters and NEAR network). Updates to this guide, reflecting any updates of the tools involved, will be paid a % of the initial bounty per each revision, up to once per month, until Stake Wars is over. Contributions in other language than English are encouraged, but considered case by case basis. | @masknetgoal634 | Jun 11 2020 | [Github](https://github.com/masknetgoal634/near-prometheus-exporter/blob/master/guide/GUIDE.md) | 1,100* | 10% | EN |
| Run a monitoring platform | Same as above | @narniec | Jun 14 2020 | [Medium](https://link.medium.com/C82TyDMXo7) | 1,000 | 10% | RU |
| Run a monitoring platform | Same as above | @wjdfx | Jun 15 2020 | [Jianshu](https://www.jianshu.com/p/c0b9d21eba3d) | 1,000 | 10% | CN |
| Run a monitoring platform | Same as above | @imnisen | Jun 23 2020 | [Github](https://github.com/dolphintwo/near-prometheus-exporter/blob/master/guide/GUIDE-compose.md) | 1,000 | 10% | CN |
| Run a monitoring platform | Same as above | @Viacheslav198 | Jul 1 2020 | [Github](https://github.com/Viacheslav198/grafana-prometheus/blob/master/alert.md) | 1,000 | 10% | IT |
| Send alerts | Create a tutorial, in the form of a video, a blogpost or Github document, that explains how to connect cloud alerts from AWS, GCP, Azure, Alibaba (and others) with your nearcore node. Contributions in other languages are encouraged, but considered case by case basis. | @wjdfx | Jun 15 2020 | [Jianshu](https://www.jianshu.com/p/0ae64d4c53aa) | 1,000 | 10% | CN |
| Send alerts | Same as above | @imnisen | Jun 23 2020 | [Github](https://github.com/dolphintwo/near-prometheus-exporter/blob/master/guide/GUIDE-alert.md) | 1,000 | 10% | CN |
| Send alerts | Same as above | @narniec | Jul 1 2020 | [Medium](https://medium.com/@narniec2020/%D1%81%D0%BE%D0%B7%D0%B4%D0%B0%D0%BD%D0%B8%D0%B5-%D0%B8-%D0%BE%D1%82%D0%BF%D1%80%D0%B0%D0%B2%D0%BA%D0%B0-%D1%83%D0%B2%D0%B5%D0%B4%D0%BE%D0%BC%D0%BB%D0%B5%D0%BD%D0%B8%D0%B5-%D0%B2-grafana-b697f3e92cc4) | 1,000 | 10% | RU |

\*Including 10% update reward

## Next Challenge
Create your warchest to dynamically keep one seat: [challenge004](challenge004.md)

## Previous Challenge
Enroll your staking pool, receive your delegation, and maintain your validator status: [challenge002](challenge002.md)

