# Stake Wars Challenge 005
Published on: July 17 2020

Automatically deploy nearcore using a CI/CD pipeline.
This challenge is designed to have validators build and deploy their own node from the source code, instead of using the precompiled binaries provided by NEAR. The result is having multiple teams that can independently build and test new releases of the node, to improve the quality and security of the network as a whole.

To participate, validators will be asked to build and deploy a specific branch/tag of [nearcore](https://github.com/nearprotocol/nearcore), and keeping their nodes up to date. 

If you already run a professional validator setup, and you have CI/CD experience, you don't need any further instructions.

If you want to test yourself and use this as a learning opportunity, the list of available tools to accomplish can be very long:
- Jenkins
- Terraform
- Gitlab
- Github Actions
- Cloud provider tools (Google Cloud Build, Azure Pipelines, AWS CodePipeline...)

You may want to read [this article](https://hackernoon.com/understanding-the-basic-concepts-of-cicd-fw4k32s1) if you need some basic knowledge on CI/CD.

## Acceptance Criteria

1. Build nearcore
2. Run tests
3. Deploy the node

### 1.Build nearcore
Subscribe to [Nearcore releases](https://github.com/nearprotocol/nearcore/releases), and automatically build the artifact from the source code as soon as there's a new version of the node. Every release is tagged as follows:
- BetaNet releases of nearcore will be mapped with the tag `x.y.z-beta`
- TestNet releases of nearcore will be mapped with the tag `x.y.z-rc`

More information is available in the official documentation, in the [contribution section](https://docs.near.org/docs/contribution/nearcore) and in the [develop section](https://docs.near.org/docs/local-setup/running-testnet#compiling-and-running-official-node-without-docker)

### 2.Run tests
Once your artifact is deployed, you may want to test its networking functionality and existing applications. Some examples can be found in [nearcore scripts](https://github.com/nearprotocol/nearcore/tree/beta/scripts)

Some examples are:
- deploy multiple nodes in `localnet` mode (an example is in the [nearup repo](https://github.com/near/nearup#spawn-local-network)) and test the capacity to connect and synchronize with the network
- attach your own local services (helper, wallet, explorer)
- send transactions, deploy/call/view smart contracts
- execute load tests and measure the performance

The outcomes of these tests will help you decide if you want to deploy this release or not. NEAR will take in serious consideration the feedback of validators who setup their own suite of tests and find bugs or errors in new releases.

### 3.Deploy the node
With the merge of [nearcore PR #2701](https://github.com/nearprotocol/nearcore/pull/2701), NEAR Protocol can run different builds of the node within the same network. You can then decide which deployment stragy adopt (e.g. Blue/Green or a Canary) and merge the node keys from the previous release to the newer one.

This process may require intermediate steps, such as database migration or state resync (by deleting the `~/.near/betanet/data` folder).

## Additional considerations
In the upcoming weeks we will introduce special builds, with tags specifically for Stake Wars Challenge 005 participants. We will measure the reactivity of valdiators to newer releases, and the capacity to identify bugs or test failures prior to the node deployment.


## Contribution Opportunities

Do you want to earn extra tokens? We have contribution opportunities available below! 

Follow the Validator channel on [NEAR Portal](https://portal.near.org/topic/validator), you will find a list of available contributions to earn tokens. In order to participate, you will have to specify:
- the title of the contribution you made
- the type of document you released
- a clear reference to the available contribution below

Once your work is reviewed, you will be added to the list below. Please note that rewards in tokens will need to pass basic KYC checks from NEAR Foundation, and comply with regulations.

**Heads up:** If you can't see/write in the Validator Portal, be sure that you filled up the [Challenge 004 contact form](https://nearprotocol1001.typeform.com/to/x4Bval), so we can send you an invitation email.

### List of available contributions
**Heads Up:** No more contributions are accepted, except the ones already posted on https://portal.near.org before October 2nd. You are free to add new ones without receiving token rewards!

| Title | Abstract                    | Contributor |  Date  | Link | NEAR Tokens | Maintenance | Language |
| -------- | ------------------------------ | ----------- | ------ | ---- | ----------- | --- | ---- |
| Release a CI Pipeline doc | Create an article or a commented Github script to explain and execute an automated building and testing of a new release of nearcore. Before starting the project request approval in the NEAR portal. | masknetgoal634 | August 5 2020 | [Github Actions](https://github.com/masknetgoal634/nearcore-deploy) | 3,500 | 15% | EN |
| Release a CI Pipeline doc | Same as above | @abellinii | August 6 2020| [Github](https://github.com/abellinii/near-ci)  | 3,500 | 15% | EN |
| Release a CI Pipeline doc | Same as above | [@Hugoo](https://github.com/Hugoo) | August 19 2020| [Blog](https://blog.hugomasclet.com/deploy-near-validator-node-kubernetes/) / [GitHub Actions](https://github.com/Gielve/NearDeploy) | 3,500 | 15% | EN |
| Release a CI Pipeline doc | Same as above | @narniec | August 23 2020| [Medium](https://medium.com/@narniec2020/ci-pipeline-jenkins-for-near-91b4ae3edde)  | 3,500 | 15% | RU |
| Release a CI Pipeline doc | Same as above | [@48cfu](https://github.com/48cfu) | September 12 2020| [Github Actions + Shell](https://github.com/48cfu/near-documentazione/blob/master/guida-localnet.md) | 3,500 | 15% | IT |
| Release a CI Pipeline doc | Same as above | [@majal](https://github.com/majal) | Sep 16 2020 | [GitHub Gist](https://gist.github.com/majal/ce091bf598ab77f69b7312791eba9af2) | 3,500 | 15% | EN |
| Release a CD Pipeline doc | Create an article and a Github document/script to explain and execute an automated deployment of a new release of nearcore. Before starting the project request approval in the NEAR portal. | @abellinii | Jul 29 2020 | [Terraform](https://github.com/abellinii/near-terraform)| 3,000 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | masknetgoal634 | August 5 2020 | [Github Actions](https://github.com/masknetgoal634/nearcore-deploy) | 3,000 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | etherscam | August 17 2020 | [Jenkins](https://github.com/etherscam/testnear) | 3,000 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | minstr22 <zainy | August 28 2020 | [Jenkins](https://github.com/minstr22/near-Protocol--ci-cd-Jenkins) | 3,000 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | @mabalaru | Sep 17 2020 | [Ansible](https://github.com/mabalaru/near/tree/master/near-ci) | 3,000 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | minstr22 <zainy | August 28 2020 | [Jenkins](https://github.com/minstr22/near-Protocol--ci-cd-Jenkins) | 3,500 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | minstr22 <zainy | August 28 2020 | [Jenkins](https://github.com/minstr22/near-Protocol--ci-cd-Jenkins) | 3,000 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | [@48cfu](https://github.com/48cfu) | September 12 2020 | [Github Actions + Shell](https://github.com/48cfu/near-documentazione/blob/master/guida-localnet.md) | 3,000 | 10% | IT |
| Release a CD Pipeline doc | Same as the above | @savelev1 | Sep 16 2020 | [Github](https://github.com/savelev1/nearcore-updater) | 3,000 | 15% | RU/EN |
| Release a CD Pipeline doc | Same as the above | @crypto-solutions(https://github.com/crypto-guys/) | Sep 16 2020 | [Github](https://github.com/crypto-guys/nearcore-continuous-delivery) | 3,000 | 15% | EN |
| Release a CD Pipeline doc | Same as the above | @crypto-solutions(https://github.com/crypto-guys/) | Sep 20 2020 | [Medium](https://medium.com/@dmytro.rozum/використання-jenkins-для-ci-cd-nearcore-7e4aa8f598a0) | 3,000 | 15% | UA |
| Release a CD Pipeline doc | Same as the above | @ama31337(https://github.com/ama31337/) | Sep 24 2020 | [Github](https://github.com/ama31337/near-update) | 3,000 | 15% | EN |
| Write a localnet tutorial | Create a tutorial on how to automatically deploy and boot your own NEAR network, using it to test a new release of nearcore. | @abellinii | August 6 2020 | [Medium](https://medium.com/@thepassivetrust/automating-ci-cd-on-a-near-network-validator-6803b3b63f2f) | 1,500 | 10% | EN |
| Write a localnet tutorial | Same as the above. | @minstr22 | August 6 2020 | [Medium](https://medium.com/@ainsleypaul7/near-jenkins-fbd42e027a93) | 1,500 | 10% | EN |
| Write a localnet tutorial | Same as the above. | [@48cfu](https://github.com/48cfu) | September 12 2020 | [Github](https://github.com/48cfu/near-documentazione/blob/master/guida-localnet.md) | 1,500 | 10% | IT |
| Write a localnet tutorial | Detailed guide on localnet | @ama31337(https://github.com/ama31337/) | September 24 2020 | [Github](https://github.com/ama31337/neartips/blob/master/manuals/running-localnet.md) | 1,500 | 10% | EN |
| Write a localnet tutorial | Same as the above | @cryptomilion | Sep 25 2020 | [Medium](https://medium.com/@shiverov/%D0%B0%D0%B2%D1%82%D0%BE%D0%BC%D0%B0%D1%82%D0%B8%D1%87%D0%BD%D0%B5-%D0%BE%D0%BD%D0%BE%D0%B2%D0%BB%D0%B5%D0%BD%D0%BD%D1%8F-nearcore-d84b22a691aa) | 1,500 | 10% | UA |


## Previous Challenge
Create your warchest to dynamically keep one seat: [challenge004](challenge004.md)
