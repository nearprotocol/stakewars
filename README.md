# Stake Wars Episode II: Return of the Validators
May 25th 2020 update: new README.md

Welcome to NEAR Stake Wars Episode II, Return of the Validators!

This page will guide you through three main aspects of this initiative:
1. What is NEAR Stake Wars
2. Your toolbox
3. What you have to do

## This Week Challenge

Create your BetaNet wallet, deploy your node, and correctly configure your staking pool. Once ready, NEAR team will delegate tokens to your pool, to have you take a seat as valdiator.
See below for details.


## :rocket: What is NEAR Stake Wars

Stake Wars is NEAR's incentivized testnet for professional validators.

NEAR’s [MainNet](https://explorer.near.org/) recently launched into its first phase, called “POA” ([see full roadmap](https://near.ai/mainnet-roadmap)). This means that a small handful of validating nodes are currently being run by the core team. In order to progress to the next phase, “MainNet: Restricted”, the operation of the network will be handed off to a large group of node operators called validators. 

The goal of Stake Wars: Episode II is to onboard those validators, test the stability of the system, and begin introducing some of the unique aspects of NEAR’s delegation in preparation for the next phase of MainNet itself.

If you want to know more about this opportunity, read the [Stake Wars Episode II blog post](https://near.org/blog/stake-wars-episode-ii/).

**TL;DR:**

Stake Wars Ep.II introduces NEAR's contract-based delegation, offering validators the opportunity to take part in the [Open Finance](https://near.org/blog/the-evolution-of-the-open-web/) ecosystem. There is a _staking pool_ reference contract on [Github](https://github.com/near/initial-contracts), ready for experimenting with these principles. Deploying the staking pool and participating in the Stake Wars will give access to NEAR's MainNet Restricted. Rewards will include 10,000 NEAR tokens/month for every validator on MainNet Restricted, plus 1 Million NEAR tokens available for contributions and community challenges. To become validators on MainNet Restricted, participants will have to accomplish technical challenges and successfully run nodes on BetaNet and TestNet. Judgment criteria will be quantitative, such as number of blocks generated and uptime; and qualitative, such as reactivity to network updates and community participation.
A Validator Advisory Board, with a selected group of professional validators, will become over time the voice of validators in the technical governance of the protocol

## :wrench: Your Toolbox

NEAR Protocol provides you multiple tools, from source code and applications, to documentation and web-based resources. As a Stake Wars participant you will use all of them.

### Github Repositories
NEAR is using two main accounts: [github.com/nearprotocol](https://github.com/nearprotocol) and [github.com/near](https://github.com/near).
To join Stake Wars you will need:
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

You will have to create a BetaNet wallet to deploy any contract (which means also the staking pool), and receive your first delegation.

You can also find a JSON RPC interface at the address [https://rpc.betanet.near.org/status](https://rpc.betanet.near.org/status) to quickly retrieve information about network, blocks, transactions and wallets. There's also a [documentation section](https://docs.near.org/docs/interaction/rpc) with the available endpoints.

As a final note, https://status.nearprotocol.com/ will give you feedback on the status of the network, and the most recent incidents.

### NEAR Community channels
Connect to other validators using the dedicated channel on [Discord](https://near.ai/validator-chat). You might join also [NEAR Validators](https://t.me/near_validators) on Telegram.

**Important:** NEAR core team will use the prefix `[CODE_RED]` if a particular message requires technical attention by validators. Some examples are new releases, hard forks and critical issues.

Overall, if you want to successfully participate in the Stake Wars, you'll have to:
- Keep an eye to Validator Announcements on Discord (there's [a dedicated channel](https://discord.gg/xsrHaCb)). It will be used to inform you about technical releases and hard forks, community challenges, contribution opportunities and other initiatives that will be valuable for you.
- Give and receive technical help here on Github, in the [issues section](https://github.com/nearprotocol/stakewars/issues).


## :trophy: What You Have to Do

As you know, validators are responsible to generate new blocks and update the state of the network. NEAR Protocol uses proof-of-stake to secure its infrastruture, so you need tokens to _stake_ in order to become a validator. The contract-based delegation, as explained in the Stake Wars Ep.II [blog post](https://near.org/blog/stake-wars-episode-ii/), will provide you stake from users who don't want to run a node but are interested to secure the network - and earn rewards with you.

### Quickstart
If you want to get your feet wet, and see if Stake Wars is for you, try this quickstart guide:
- Create a **BetaNet** account using our hosted wallet [here](https://wallet.betanet.near.org).
- Spin up an Ubuntu/Debian VPS and install [nearup](https://github.com/near/nearup).
- Deploy your Staking Pool contract using the [initial-contracts repo](https://github.com/near/initial-contracts/).
- Once you are ready, fill up [this form](https://nearprotocol1001.typeform.com/to/TvvOMf) to enroll in the Stake Wars to receive updates on the program and simplify our evaluation process.

As soon as we have a new slot available, we will _delegate_ enough BetaNet tokens to your pool, for a few days.
If your node will not be running, or your staking pool was misconfigured, we will get in touch with you. We'll want to know more about your validator experience, and evaluate if you are ready to be included in the Stake Wars TestNet roster.

### List of Validator Challenges
We will publish new challenges for validators on a regular basis. Every challenge will have a progressive number, with an increasing level of difficulty. The acceptance criteria will provide high-level indications, and some of these challenges will list which previous challenges should be completed before.

List of community challenges:

1. May 25th 2020, [Challenge 001](challenges/challenge001.md): Deploy your node.
2. May 25th 2020, [Challenge 002](challenges/challenge002.md): Deploy your Staking Pool.
