# Stake Wars 2.0 Guidelines

## Format
This new phase of Stake Wars will test stake delegation to validators. The winners will be those validators who deploy reliable nodes and challenge the boundaries of Proof of Stake, by offering innovative delegation solutions that can attract the favor of Stake Wars participants. The larger the stake you will attract from real users, the bigger your success during the Stake Wars and beyond, when the MainNet will be running on your nodes.
NEAR Protocol will promote this initiative across all its channels, by progressively releasing Stake Wars to every user who owns a TestNet wallet. This will be a unique opportunity to attract NEAR adopters on your platform and grow your validator business.

This is not going to be easy: we will deploy hard forks constantly, we will test your team reactivity in case of network issues, we will run benchmarks at the network level, and we will measure the strenght of the participants as a whole.
However, Stake Wars is not about doing things right or wrong. It is mostly about learning how smart contract-based delegation works, and get ready to run NEAR together.

## Rules
There are no strict rules, rather guidelines and examples. Judges on NEAR side will rate your participation, and will update the [leaderboard](LEADERBOARD.md) every week.

_If you are acting in good faith, you’re almost assuredly good. If you’re acting in bad faith, moderators will exclude you from the competition._

Here are some examples and we will trust you to follow the spirit of the law:

### DO…
* Check out the QA scenarios and run them on your system.
* Read the documentation, and open GitHub issues if you find ways to improve it.
* Run your node like it's a production environment and you are staking with real funds.
* Run any kind of benchmark you think is valuable.
* Try staking in all sorts of ways
  * You will be rewarded for creativity, effort and magnitude of the submission
* Write reports on your experience and send them to us publicly.
* Complain about things that need fixing or cleanup!

### DON’T…
* Do anything illegal in order to hack our stuff.
* Hack other peoples’ computers or use attack vectors that are outside of our control
  * For example, if you add a keylogger to someone’s computer, that is showing a flaw in their Operational Security not a flaw in the system we’re building.
* Harass people in any way. This includes community members and team members.
* Be a jerk.


## Our Goals for this
We want...

* You to build things that solve your own problems as future validators on the network
* To put the last year or so of hard work in front of our community to try it out
* To find glaring flaws in our systems, designs and code
* To learn what your needs are as part of the validator community

## Additional guidelines to join TestNet
We are happy to have as many professional validators and distinguished BetaNet node operators to TestNet as we can. However, TestNet network requires additional attention, as we have applications and developers actively using it - which means we can't afford to have nodes offline or not updated.

Stake Wars organizers will reach out directly, proposing to shut down your BetaNet node, and receive delegated tokens on TestNet. There are no specific requirements, however the evaluation will be based on these factors (not in order of importance):
* your staking pool uptime is in the 90th percentile of the [leaderboard](LEADERBOARD.md)
* your pool was running for a substantial number of epochs
* you successfully completed the Stake Wars challenges
* you are active in the community channels (Discord and the [portal](https://portal.near.org))
* you actively updated your node and successfully completed BetaNet migrations on Tuesdays
* you are already a successful validator on other PoS networks
* you contributed to `nearcore` and other NEAR repositories (submitting issues, finding bugs, improving docs...)

In practical terms, to join TestNet you will have to:
1. deploy your TestNet staking pool, using the staking pool factory [smart contract](https://explorer.testnet.near.org/accounts/stakingpool)
2. build and run your TestNet node
3. submit your information to [this form](https://nearprotocol1001.typeform.com/to/x4Bval). Specify both your BetaNet and TestNet pools, and don't forget to disclose your Github and Discord/Telegram IDs, so it's easier for us to contact you in case of issues

You can instantly deploy the staking pool factory with near-shell, using the command:
```
near call stakingpool create_staking_pool '{"staking_pool_id":"<POOL_ID>", "owner_id":"<OWNER_ID>", "stake_public_key":"<VALIDATOR_KEY>", "reward_fee_fraction": {"numerator": <X>, "denominator": <Y>}}' --account_id <OWNER_ID> --amount 30 --gas 300000000000000
```
Where:
* `stakingpool` is the staking pool factory contract mentioned above
* `POOL_ID` is the name of the staking pool contract. If your validator name is `nearkat` the result will be `nearkat.stakingpool`
* `OWNER_ID` is the owner of the pool, who's authorized to change the stake public key and the fees
* `VALIDATOR_KEY` is the public key found in the file `~/.near/testnet/validator_key.json` on the machine running the node
* `{"numerator": <X>, "denominator": <Y>}` set the validator fees. To set 10% of fees x=10 and y=100
* `--amount 30` attaches 30 $NEAR to the transaction, as a reserve to pay the contract storage
* `--gas 300000000000000` specifies the gas for the transaction (optional)

**Heads up:** be sure that your validator node and your staking pool have the same ID, as explained [here](troubleshooting.md#11-my-validator-is-in-the-current_validators-set-but-its-not-producing-blocks).

The main differences with BetaNet will be:
- less frequent updates
- a different branch of nearcore: [stable](https://github.com/nearprotocol/nearcore/tree/stable)
- longer epochs (43,200 blocks instead of 10,000)

**Heads up:** TestNet is a permissionless network, so you can deploy your validator node and its staking pool today. However, Stake Wars organizers will delegate TestNet tokens to your pool only after reaching out and discussing your commitment in running the network.

