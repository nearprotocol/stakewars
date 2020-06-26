# UPDATES

## UPDATE June 29th

* Added the guidelines to join TestNet! See them [here](GUIDELINES.md)

## UPDATE June 22nd

* Added the Challenge 004, to learn how to dynamically adjust the stake of a pool: [Challenge004](/challenges/challenge004.md)

## UPDATE June 17th

* Leaderboard modified: now it counts only expected blocks and uptime.
* Contributions listed in the challenges, they are worth to be read!
* Another BetaNet update was successfully completed, with minimal downtime. Congrats validators!

## UPDATE June 8th

* New challenge available [here](challenges/challenge003.md)! Build your monitoring platform and send alert emails in case of issues. Add `stefano@near.org` to your email recipients to receive extra funds for delegation.

## UPDATE May 28th

**We sent 75,000 tokens to all applicants, excluding obvious duplicates**
We received exceptional interest by the community of validators, with 300+ applications and 200+ PRs). We are processing all of them, sending 75,000 BetaNet tokens as promised, while making sure the network runs with no issues. As explained in the [docs](https://docs.near.org/docs/validator/economics), there are 100 seats per shard, so BetaNet is running at capacity with a cost per seat right above 75.000 tokens - which means that our contribution will not be sufficient.
How to solve this issue? Activate your network, and ask other validators to delegate to your staking pool. There are many of them with 100k+ tokens who can delegate 5-10k of their stake to you.

The plan is to start transitioning the first batch of validators to TestNet. As soon as that happens, the seat price will go down, allowing more folks to join BetaNet.
In the meantime, apply for the [contributions](challenges/challenge001.md) as they are worth real NEAR tokens, and keep an eye on the seat price, using the command `near validators current`.

## UPDATE May 22nd

* New `near-shell` 
* BetaNet Incident #0004

### New near-shell

The release `0.24.0` of `near-shell` is live! Update by using `npm` and check with `near --version` if you are up to date. 

### BetaNet Incident #0004

**Incident Abstract:**

The entire betanet crashed on May 22 at 09:36:47 UTC due to a bug that resulted in storage inconsistent state for every node.

**Incident Description:**

Betanet nodes crashed with the following error

```
StorageInconsistentState("Account stakepool_buildlinks.betanet with max of stakes 0 is not found")
```

It is [a known issue](https://github.com/nearprotocol/nearcore/issues/2687) that has already been [fixed](https://github.com/nearprotocol/nearcore/pull/2688) in master of nearcore, but the fix was not included in the last betanet release. 

**Remediation:**

1. Upon discovery of the issue, @Vlad Frolov and @Illia Polosukhin did a hard fork of the network with the same binary to restart the network.
2. After @Sandi Fatic and @Bowen Wang joined, we realized that the issue is already fixed in master so we release a new version of betanet with the fix.
3. We are going to improve on the incident communication internally and externally. We use will experiment a better integration with slack to effectively communicate when an incident like this occurs. 
4. We are also going to improve how critical issues get fixed and deployed to the live networks. In this case, the fix didn't get included in last Tuesday's BetaNet release because it was considered unlikely to happen. From now on, bugs that can cause the entire network to go down will be communicated differently, and the fixes deployed as soon as possible, unless it is technically impossible for the bug to occur (due to current restrictions to other functionalities).
5. We will organize and implement a training to handle such incidents across more team members, instead of having time-zone constraints and waiting for certain team members to wake up. Specifically, this action will teach how to debug, make a decision about doing a hard-fork or not doing it, and, finally, how to deploy the fixes.

If you have questions and you want to know more, reach out on [Discord](https://discord.gg/jWynGsn).


## UPDATE May 19th

* Stake Wars Ep.II is live!
* Deploy your Staking Pool Contract

### Stake Wars Episode II is live!

We officially launched Stake Wars Episode II: The Return of Valdiators. This new phase will test the contract-based delegation of NEAR Protocol, and will offer new ways for validators to differentiate their services. More information on the [official blog post](https://near.org/blog/stake-wars-episode-ii/). Remember to update your validator entry on the [VALIDATORS.md](VALIDATORS.md), adding the link to your deployed staking pool.

### Deploy your Staking Pool Contract

To enter this second phase of Stake Wars, you will have to:
- unstake your tokens from your node
- deploy the staking pool contract (you can find more info in the [initial-contracts Github repo](https://github.com/near/initial-contracts))
- update the [VALIDATORS.md](VALIDATORS.md) file with the staking pool address
- delegate any original stake to the new contract

If you lost your tokens, or you just joined and you don't have enough stake, reply in the [issue 289](https://github.com/nearprotocol/stakewars/issues/289) with your staking pool address. 


## UPDATE Apr 29th

* New `near-shell`
* New `nearcore`, improved consensus and networking for higher stability
* BetaNet Incident #0002

### BetaNet Incident #0002

NEAR's planned weekly update of BetaNet failed, the network didn't produce new blocks from 00:55 GMT to 7:10 GMT

**Incident Description:**

- NEAR began the scheduled weekly update of its `nearcore` node at 00:00 GMT
- The deployment pipeline completed successfully at 00:55 GMT, with the release `Version: 0.4.13, Build: 38d54ba7-modified`
- This update needed a hard-fork, with a new `genesis.json` file containing `"genesis_height": "3818145"`. Previous releases were not able to sync to any block higher than `3818145`
- As planned, all NEAR's BetaNet nodes switched to this new release, and the new genesis block was broadcasted through NEAR's `boot nodes`
- At 00:56 GM the Core Team informed NEAR Validators on Telegram and Discord that all nodes had to be updated and restarted asap
- This update could be automatically triggered by stopping and starting `nearup` (only for `nearup` users), or in case of automated deployment pipeline, by downloading and compiling `nearcore` from the branch `beta`
- Multiple nodes were correctly updated, however the network reached 66% of the total stake only at 7:10 GMT, after 6 hours and 20 minutes from the advertised release messages
- At 7:10 GMT the network reached a quorum, with a throughput between `0.10 bps` and `0.20 bps (blocks per second)`
- At 10:49 GMT, with block 3822000, the protocol unstaked the validators which failed to update on time and the ones that were running the old release, bringing back BetaNet in its normal state

**Remediation:**

- An investigation is in progress: NEAR's Core Team is contacting all the node operators that were validators at 00:00 GMT, to troubleshoot the upgrade process and identify the issues encountered
- As soon as the investigation is complete, this remediation will be extended with the introduced measures in terms of bug fixes, new features, documentation and communication

## UPDATE Apr 21st

* Released [Road to Mainnet](https://near.ai/mainnet-roadmap)
* Released `nearup` to the validator community
* Updated the Stake Wars 2.0 branch on GitHub

## UPDATE Apr 14th

* Stake Wars has been redesigned to involve end-users, and test NEAR smart contracts delegation
* The reference network for Stake Wars is now `betanet`. We suggest you to install and run the new node via [nearup](https://github.com/near/nearup)
* To obtain some betanet tokens and become validator, send a request via [this form](https://forms.gle/5KabPsD4BefR6nv68).

## UPDATE Nov 12th

* Just update the issue templates. We are now measuring severity internally since the templates were just confusing.
* The simplified submission process is: just follow the template, and we will add label.
* If we add the `help wanted` tag, you can fix it and receive tokens as part of the competition as well

## UPDATE Nov 20th

* sudo is no longer needed if you're on Mac, or if you're on linux and can run docker without sudo.

## UPDATE Nov 22nd

* We're using Github to collect genesis for this week. Please don't submit your keys for genesis if you can't make the call. Put in a PR againts accounts.csv with your changes.

## UPDATE Dec 13th

* We're restructuring the program to focus on finding bugs that are based on QA scenarios rather than security breaches and attacks.
* You can find these in `scenarios.md` in this repo. [HERE](./scenarios.md)