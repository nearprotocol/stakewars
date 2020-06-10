# Stake Wars Troubleshooting

This document will try to address the most common issues that you may have while running Stake Wars and setting up your node.
Contributions and corrections are welcome!

## Table of contents

1. My validator is in the `current_validators` set, but it's not producing blocks
1. I used `near send` instead of `near call` to a staking pool
1. I used `near stake` instead of `near call` to stake funds to a pool
1. I see a `Telemetry data` error in the logs
1. I receive a `Peer stream error` while syncing with other nodes
1. I see a `type: 'UntypedError'` if I try to use near-shell
1. I receive a `KeyNotFound` error if I try to use near-shell
1. I receive a timeout error from RPC after I sent a command
1. I had a wallet created on [nearprotocol.com](http://nearprotocol.com) and now I can't see it anymore
1. I receive a `Server error: Timeout` when I use near-shell
1. I receive a ` ` when I withdraw funds from my staking pool
1. I set up the wrong owner to my staking pool, are funds lost?
1. My node seems stuck at 98% of the sync, without progressing
1. My node is stuck at zero peers



```
{
  type: 'KeyNotFound'
}
```

```
panic_msg: "panicked at 'The new total balance should not be less than the old total balance', src/lib.rs:509:9"
```

## 4. I see a `Telemetry data` error in the logs

```
Jun 07 21:26:49.489  INFO telemetry: Telemetry data could not be sent due to: protocol error: not a result of an error
```

## Additional resources

- Join the Validator chat at [https://near.chat].
- Ask questions on Stack Overflow, using the tags [nearprotocol](https://stackoverflow.com/questions/tagged/nearprotocol) and [nearprotocol-validator](https://stackoverflow.com/questions/tagged/nearprotocol-validator).
- Open an issue (or ask for a new feature!) directly from [here](issues).