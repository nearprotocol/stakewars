# UPDATES

## UPDATE Apr 29th

* New `near-shell`
* New `nearcore`, improved consensus and networking for higher stability
* BetaNet Incident #0002

**Incident Abstract:**

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