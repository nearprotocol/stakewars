# Stake Wars 2.0 Test Scenarios

## Basic Participation Needs
This second phase of NEAR Stake Wars will try to break the delegation of tokens to Validators.
Validators still have to properly run their nodes infrastructure, and get familiar with the activities below:
1. Professionally deploy and run the node
2. Update the node software
3. Deploy the Delegation smart contract
4. Enroll your Validator
5. Update the Delegation smart contract

### 1.Professionally deploy and run the node
- Pull `nearcore` from [https://github.com/nearprotocol/nearcore](https://github.com/nearprotocol/nearcore). To run on BetaNet you have to checkout on the `beta` branch
- Build `nearcore` launching `make release`
- Install `nearup` from [https://github.com/near/nearup](https://github.com/near/nearup)
- Run the node from your compiled binary, properly using `--nodocker` and `--binary-path`
- Monitor your node performance, by measuring `num_expected_blocks` and `num_produced_blocks` from the `validators` method on the JSON RPC - [here more info](https://docs.nearprotocol.com/docs/interaction/rpc)
- Design and build your own redundancy/availability solutions for your nodes

**Heads up:** running a node is up to your experience, these are not requirements. As an example, you may build your own docker image, or avoid using `nearup` completely.

### 2.Update the node software
- Download and deploy the weekly release of `nearcore` from the `beta` branch. It is normally merged by NEAR core team on Monday at 5:30pm Pacific and deployed on `DevNet`.
- In case of a planned hard-fork, restart the node with the new release at the programmed time
- Update the node without missing more than 10% of the `num_expected_blocks` (to avoid being kicked out in the next epoch)
- Promptly Re-stake if your node misses the target and will be kicked out

**Heads up:** as of today, BetaNet goes through a reset and hard-fork every Tuesday at 6pm Pacific. More details can be found in the documentation pages at [docs.near.org/validator/staking-overview](https://docs.near.org/docs/validator/staking-overview)


### 3.Deploy the Delegation smart contract
_Note: this part is still work in progress_
- Clone and compile the [Staking Pool contract](https://github.com/near/initial-contracts/tree/master) on the operator machine
- Deploy the contract with `near-shell`
- Configure the contract with the exact node id and its validator public key
- Stake delegating your BetaNet funds to the contract


### 4. Enroll your Validator
Once your contract is ready, you can issue a pull request to the file [VALIDATORS.md](VALIDATORS.md), including your information at the bottom of the list. This file will automatically enroll your node in the Stake Wars 2.0, and will help users to find your staking pool.


## Advanced Participation Opportunities
Stake Wars is an opportunity to explore the potential of smart contracts for delegation. Ideally, every Validator could release its very own staking pool contract, with unique features in terms of:
- Lock duration and liquidity
- Variable validation fees
- Compounding rewards
- Governance and community voting
- Bounties and community projects
- Rewarded custody services

Our goal, as validators, is to make end-users used with this new way of staking tokens.
Their actions will be quite basic:
- deposit and withdraw tokens
- lock and unlock tokens in the staking
- measure their rewards


## Stake Wars Rewards
Please note that Stake Wars 2.0 is not rewarding any basic scenarios, as they were already tested and rewarded during the first phase. At this stage, we suggest participating Validators to test the delegation via smart contracts and, once ready, leverage this opportunity to join the MainNet Restricted along with their users.
