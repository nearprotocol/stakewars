# Stake Wars Guidelines

## Format

### Weekly calls on Mondays

* Time will alternate each week to accommodate different hemispheres
* We establish a new genesis each week together and start nodes on the calls
* You do stuff during the week and submit here using the issue templates
* We assign points at the end of the week and reveal the leaderboard each Monday
* At the end of the competition, we will convert the points to tokens and perform KYC required by your country of residence to assign them to you.

### Submission Types

* P3 (Test) - lowest severity of bug or test that you can run. 
    Example might be a bug you found in our scripts that start a node.
* P2 (Bend) - medium severity bug or test.
    Example might be a bug you find in the actual core code that causes the system to lag (but not stop)
* P1 (Break) - highest severity bug or epic thing you’ve built.
    Example might be a consensus breaking bug. 
    Example might be a fully functioning delegation suite of smart contracts working on the system

## Rules

This program is hard to create rules for, so we’re giving you guidelines and examples. Judges on our side will rate your submissions and assign points.

*If you are acting in good faith, you’re almost assuredly good. If you’re acting in bad faith, moderators will ban you from the competition.*

Here are some examples and we will trust you to follow the spirit of the law:

### DO…

* Run a node or as many nodes as you can figure out how to run.
* Run any kind of benchmark you think is valuable
* Try to break our tools and systems.
* Try staking in all sorts of ways.
  * You will be rewarded for creativity, effort and magnitude of the submission. (For instance, if you get delegation to work for a bunch of small accounts)
* Collude with other validators to try to sign bad transactions or to validate bad blocks and try to corrupt the system
* Review the code for vulnerabilities and try to exploit them
* Write reports on your experience and send them to us publicly
* Complain about things that need fixing or cleanup!

### DON’T…

* Do anything illegal to hack our stuff.
* Hack other peoples’ computers or use attack vectors that are outside of our control
* For example, if you add a keylogger to someone’s computer, that is showing a flaw in their Operational Security not a flaw in the system we’re building
* Harass people in any way. This includes community members and team members.
* Be a jerk

## Points Overview

In order to get points, you will need to submit an issue on GitHub for each problem you find and each test you run. ([https://github.com/nearprotocol/stakewars](https://github.com/nearprotocol/stakewars))

**GET STARTED:** You can all get the basic points for running a node (or nodes) and running an interesting benchmark.

Since we hope all of you will do this, we’ve created an issue for this in the main repo: [https://github.com/nearprotocol/stakewars/issues/4](https://github.com/nearprotocol/stakewars/issues/4)

### Here’s how points work

* You submit issues based on what you run into while attempting to run nodes to the GitHub repo identified above.
* We (a rotating panel of reviewers) assign points to whoever submitted based on the severity of bugs.
* The identity of the submitter must be associated with the submission by the end of the comp to be valid. Otherwise no points.
* We update the leaderboard at least every week, but it may be more frequently based on submissions.

## Agreement and edge cases

* In addition, you will have to sign a program and token agreement for the reward of tokens based on points. This will happen at some point during the competition.
* You CAN get multiple rewards for running multiple nodes, BUT only for giving us valuable information about problems in our system. It’s still dependent on the severity of the bug or the value of the test you’ve created. For example…
  * If you create a bunch of nodes and then just leave them up. No points.
  * If you create a bunch of nodes to test a delegation contract you created, very valuable. Much points. Very wow.
* We’re not making it public what the translation from points to tokens is. This is mainly because we don’t want to commit to specific numbers upfront and be wrong.
* The point system allows us to create weights for bugs and tests. Our goal is to do this fairly and equitably. No funny business.

## Our Goals for this

We want...

* You to break our system in a way that we can permanently fix it!
* You to build things that solve your own problems as future validators on the network
* To put the last year or so of hard work in front of our community to try it out
* To find glaring flaws in our systems, designs and code
* To learn what your needs are as part of the validator community
