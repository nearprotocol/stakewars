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


## Contribution Opportunities

Do you want to earn extra tokens? We have contribution opportunities available below! 

Reply to the challenges application thread on [Github](https://github.com/nearprotocol/stakewars/issues/350) specifying:
- which contribution you want to do, and the link to the challenge
- the type of document you will release
- the date when you will publish the content (you can postpone a few days, no worries)

Once your work is done, you will be added to the list below. Please note that rewards in tokens will need to pass basic KYC checks from NEAR Foundation, and comply with regulations.

### List of available contributions

| Abstract | Description                    | Contributor | Due Date | Link | NEAR Tokens | Maintenance | Language |
| -------- | ------------------------------ | ----------- | -------- | ---- | ----------- | --- | ---- |
| Run a monitoring platform | Create a tutorial, in the form of a video, a blogpost or Github document, that explains how to monitor the health of your node (general parameters, nearcore parameters and NEAR network). Updates to this guide, reflecting any updates of the tools involved, will be paid a % of the initial bounty per each revision, up to once per month, until Stake Wars is over. Contributions in other language than English are encouraged, but considered case by case basis. | - | - | - | 1000 | 10% | Any |
| Send alerts | Create a tutorial, in the form of a video, a blogpost or Github document, that explains how to connect cloud alerts from AWS, GCP, Azure, Alibaba (and others) with your nearcore node. Contributions in other languages are encouraged, but considered case by case basis. | - | - | - | 1000 | 10% | Any |


