# Stake Wars Episode II: Return of the Validators
July 22nd 2020 update: added an explanation of the available challenges

Welcome to NEAR Stake Wars Episode II, Return of the Validators!

This page will guide you through three main aspects of this initiative:
1. What is NEAR Stake Wars
2. Your toolbox
3. What you have to do

## :information_desk_person: In a Few Words

Stake Wars is the program that accelerates your path to become a validator on NEAR Protocol. It is structured in technical challenges of increasing difficulty, giving you the opportunity of "learning by doing". Validators that will complete all the challenges will prove to be ready to join [MainNet Restricted](https://near.org/blog/near-mainnet-genesis/) and will be recognized as early supporters of NEAR Protocol.

_List of available challenges:_
- [001](challenges/challenge001.md): Create your BetaNet wallet, deploy your node, and correctly configure your staking pool.
- [002](challenges/challenge002.md): Enroll your staking pool, receive your delegation, and maintain your validator status!
- [003](challenges/challenge003.md): Monitor your node health, and send an automated email in case of issues.
- [004](challenges/challenge004.md): Create a warchest of staked tokens, and dynamically maintain no more than one validator seat.
- [005](challenges/challenge005.md): Automatically deploy nearcore using a CI/CD pipeline.

- [00x](challenges/challenge00x.md): _Challenge other validators with something that validators should be doing, just use this template!_




## :rocket: What is NEAR Stake Wars

Stake Wars is NEAR's incentivized testnet for professional validators.

NEAR’s [MainNet](https://explorer.near.org/) recently launched into its first phase, called “POA” ([see full roadmap](https://near.org/blog/mainnet-roadmap/)). This means that a small handful of validating nodes are currently being run by the core team. In order to progress to the next phase, “MainNet: Restricted”, the operation of the network will be handed off to a large group of node operators called validators. 

The goal of Stake Wars: Episode II is to onboard those validators, test the stability of the system, and begin introducing some of the unique aspects of NEAR’s delegation in preparation for the next phase of MainNet itself.

If you want to know more about this opportunity, read the [Stake Wars Episode II blog post](https://near.org/blog/stake-wars-episode-ii/).

**Blogpost TL;DR:**

Stake Wars Ep.II introduces NEAR's contract-based delegation, offering validators the opportunity to take part in the [Open Finance](https://near.org/blog/the-evolution-of-the-open-web/) ecosystem. There is a _staking pool_ reference contract on [Github](https://github.com/near/initial-contracts), ready for experimenting with these principles. Deploying the staking pool and participating in the Stake Wars will give access to NEAR's MainNet Restricted. Rewards will include 10,000 NEAR tokens/month for every validator on MainNet Restricted, plus 1 Million NEAR tokens available for contributions and community challenges. To become validators on MainNet Restricted, participants will have to accomplish technical challenges and successfully run nodes on BetaNet and TestNet. Judgment criteria will be quantitative, such as number of blocks generated and uptime; and qualitative, such as reactivity to network updates and community participation.
A Validator Advisory Board, with a selected group of professional validators, will become over time the voice of validators in the technical governance of the protocol




## :wrench: Your Toolbox

NEAR Protocol provides you multiple tools such as Github repositories,  applications, documentation and web-based resources. As a Stake Wars participant you will use all of them.

### Github Repositories
NEAR is using two main accounts: [github.com/nearprotocol](https://github.com/nearprotocol) and [github.com/near](https://github.com/near).
To join Stake Wars you will use:
- [nearup](https://github.com/near/nearup), public scripts to launch NEAR Protocol devnet, betanet and testnet nodes
- [nearcore](https://github.com/nearprotocol/nearcore), the reference client for NEAR Protocol
- [near-shell](https://github.com/near/near-shell), the general purpose command line tools for interacting with NEAR Protocol
- [initial-contracts](https://github.com/near/initial-contracts), where you can find the staking pool

### NEAR Documentation
Most of the technical documentation is available at [docs.near.org](https://docs.near.org). An [entire section](https://docs.near.org/docs/validator/staking-overview) is dedicated to validators.

### NEAR Online Resources
The website provides a block explorer and a web wallet:
- BetaNet web wallet: [wallet.betanet.near.org](https://wallet.betanet.near.org)
- BetaNet explorer: [explorer.betanet.near.org](https://explorer.betanet.near.org/)

You will need a BetaNet wallet to deploy your staking pool and receive your first delegation.

You can also use the JSON RPC interface at the address [https://rpc.betanet.near.org/status](https://rpc.betanet.near.org/status) to quickly retrieve information about network, blocks, transactions and wallets. There's also a [documentation section](https://docs.near.org/docs/interaction/rpc) with the available endpoints.

As a final note, https://status.nearprotocol.com/ will give you feedback on the status of the network, and the most recent incidents.

### NEAR Community channels
Connect to other validators using the dedicated channel on [Discord](https://near.ai/validator-chat). You might join also [NEAR Validators](https://t.me/near_validators) on Telegram.

**Important:** NEAR core team will use the prefix `[CODE_RED]` if a particular message requires technical attention by validators. Some examples are new releases, hard forks and critical issues.

Overall, if you want to successfully participate in the Stake Wars, you'll have to:
- Keep an eye to Validator Announcements on Discord (there's [a dedicated channel](https://discord.gg/xsrHaCb)). It will be used to inform you about technical releases and hard forks, community challenges, contribution opportunities and other initiatives that will be valuable for you.
- Give and receive technical help here on Github, in the [issues section](https://github.com/nearprotocol/stakewars/issues).

### Other files is in this repo
* [GUIDELINES.md](GUIDELINES.md): some basic rules to follow, and what to do when you are invited to join TestNet
* [updates.md](updates.md): good to catch up if you missed the last few weeks
* [VALIDATORS.md](VALIDATORS.md): who else is participating in the Stake Wars
* [LEADERBOARD.md](LEADERBOARD.md): how validators are doing in terms of uptime
* [troubleshooting.md](troubleshooting.md): a colletion of the most common issues
* [staking_cheatsheet.md](staking_cheatsheet.md): a list of the most common commands to manage the staking pool
* [scenarios.md](scenarios.md): some high-level examples of what you will have to do

...and the [challenges](challenges/) folder...




## :trophy: What You Have to Do

As you know, validators are responsible to generate new blocks to update the state of the network. NEAR Protocol uses proof-of-stake to secure its infrastruture, so you have to _stake_ tokens to become a validator. The contract-based delegation, as explained in the Stake Wars Ep.II [blog post](https://near.org/blog/stake-wars-episode-ii/), will provide you stake from users who don't want to run a node but are interested to secure the network - and earn rewards with you.

### Quickstart
If you want to get your feet wet, and see if Stake Wars is for you, try this quickstart guide:
- Create a **BetaNet** account using our hosted wallet [here](https://wallet.betanet.near.org).
- Spin up an Ubuntu/Debian VPS and run [nearup](https://github.com/near/nearup).
- Deploy your staking pool using the [Staking Pool Factory](https://near-examples.github.io/staking-pool-factory/)
- Submit up [this form](https://nearprotocol1001.typeform.com/to/TvvOMf) to enroll in the Stake Wars.

At the moment of writing (May 28) we received an exceptional interest by the community of validators (300+ applications, 200+ PRs). We are processing all of them, sending 75,000 BetaNet tokens as promised, while making sure the network runs with no issues. As explained in the [docs](https://docs.near.org/docs/validator/economics), there are 100 seats per shard, so BetaNet is running at capacity with a cost per seat right above 75.000 tokens.
How to solve this? Activate your network, and ask other validators to delegate to your staking pool. There are many of them with 100k+ tokens who can delegate 5-10k from their stake to your pool.
The plan is to start transitioning the first batch of validators to TestNet. As soon as that happens, the seat price will go down, allowing more folks to join BetaNet.
In the meantime, give a look to the available contributions, and keep an eye on the seat price, using the command `near validators current`.


### List of Validator Challenges
We will publish new challenges for validators on a regular basis. Every challenge will have a progressive number, with an increasing level of difficulty. The acceptance criteria will provide high-level indications, and some of these challenges may list previous challenges as a requirement.

List of challenges:

1. May 25th 2020, [Challenge 001](challenges/challenge001.md)
  Deploy your node and your staking pool
2. May 25th 2020, [Challenge 002](challenges/challenge002.md)
  Become a validator and manage your seat
3. June 8th 2020, [Challenge 003](challenges/challenge003.md)
  Monitor your node and setup automated alerts
4. June 22nd 2020, [Challenge 004](challenges/challenge004.md)
  Dynamically adjust your stake
5. July 17 2020, [Challenge 005](challenges/challenge005.md)
  Automatically deploy nearcore using a CI/CD pipeline.
