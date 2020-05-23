# May 19th 2020 update - Stake Wars Episode II: Return of the Validators

Welcome to NEAR Stake Wars Episode II, Return of the Validators!

This page will guide you through three main aspects of this initiative:
1. What is NEAR Stake Wars
2. Your toolbox
3. What you have to do

## What is NEAR Stake Wars

Stake Wars is NEAR's incentivized testnet for professional validators.

NEAR’s [MainNet](https://explorer.near.org/) recently launched into its first phase, called “POA” ([see full roadmap](https://near.ai/mainnet-roadmap)). This means that a small handful of validating nodes are currently being run by the core team. In order to progress to the next phase, “MainNet: Restricted”, the operation of the network will be handed off to a large group of node operators called validators. 

The goal of Stake Wars: Episode II is to onboard those validators, test the stability of the system, and begin introducing some of the unique aspects of NEAR’s delegation in preparation for the next phase of MainNet itself.

If you want to know more about this opportunity, read the [Stake Wars Episode II blog post](https://near.org/blog/stake-wars-episode-ii/).

**TL;DR:**
Stake Wars Ep.II introduces NEAR's contract-based delegation, enabling validators to take part in the [Open Finance](https://near.org/blog/the-evolution-of-the-open-web/) ecosystem. There is a _staking pool_ reference contract on [Github](https://github.com/near/initial-contracts), ready for experimenting with these principles. Deploying the staking pool and participating in the Stake Wars will give access to NEAR's MainNet Restricted. Rewards will include 10,000 NEAR tokens/month for every validator on MainNet Restricted, plus 1 Million NEAR tokens available for contributions and community challenges. To become validators on MainNet Restricted, participants will have to accomplish technical challenges and successfully run nodes on BetaNet and TestNet. Judgment criteria will be quantitative, such as number of blocks generated and uptime; and qualitative, such as reactivity to network updates and community participation.
A Validator Advisory Board, with a selected group of professional validators, will become over time the voice of validators in the technical governance of the protocol

## Your Toolbox

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

You will have to create a BetaNet wallet to deploy the staking pool, and receive your first delegation.

You can also find a JSON RPC interface at the address [https://rpc.betanet.near.org/status](https://rpc.betanet.near.org/status) to quickly retrieve information about network, blocks, transactions and wallets. There's also a [documentation section](https://docs.near.org/docs/interaction/rpc) with the available endpoints.

As a final note, https://status.nearprotocol.com/ will give you feedback on the status of the network, and the most recent incidents.

### NEAR Community channels
Connect to other validators using the dedicated channel on [Discord](https://near.ai/validator-chat). You might join also [NEAR Validators](https://t.me/near_validators) on Telegram.
Overall, if you want to successfully participate in the Stake Wars, you'll have to:
- Keep an eye to Validator Announcements on Discord (there's [a channel](https://discord.gg/xsrHaCb) only for that). It will be used to inform you about technical releases and hard forks, community challenges, contribution opportunities and other initiatives that will be valuable for you.
- Give and receive technical help here on Github, in the [issues section](https://github.com/nearprotocol/stakewars/issues).


## What You Have to Do

As you know very well, validators are responsible to generate new blocks and update the state of the network. NEAR Protocol uses proof-of-stake to secure its infrastruture, so you need tokens to _stake_ in order to become a validator. The contract-based delegation, as explained in the Stake Wars Ep.II [blog post](https://near.org/blog/stake-wars-episode-ii/), will provide you stake from users who can't run a node but are interested to secure the network - and earn rewards from it.

### Quickstart
If you want to get your feet wet, and see if Stake Wars is for you, try this quickstart guide:
- Create a **BetaNet** account using our hosted wallet [here](https://wallet.betanet.near.org).
- Spin up an Ubuntu/Debian VPS and install [nearup](https://github.com/near/nearup).
- Deploy your Staking Pool contract using the [initial-contracts repo](https://github.com/near/initial-contracts/). Be careful: you will have to understand which is the right validator key to use!
- Once you are ready, fill up [this form](https://nearprotocol1001.typeform.com/to/TvvOMf) to enroll in the Stake Wars.

As soon as we have a new slot available, we will _delegate_ 75,000 BetaNet tokens to your pool, for seven days.
After this trial period, we will unstake the tokens and we'll get in touch with you. We'll want to know more about your validator experience, and include you in the official Stake Wars roster. 


### 