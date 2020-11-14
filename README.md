<div align="center">

# SquiFi

pronounced `sqwi-fy`

![squifi image](squifi.jpg)

</div>

SquiFi is a crowd-funding protocol built on [solana](https://solana.com/).

Squifi has multiple types of funds a user can create. All funds have a max that when reached depositors will not be able to deposit.

- Raise
  - A raise is  exactly what it sounds like. A way to raise funds for a single cause. There are countless use cases for this type of fund a few are:
    - Health Bills
    - Tuition
  - Raises can also be used to help pool money with your friends for a group trip.

- Pool
  - A pool can be private or public. There are associated shares to a raise. A pool is seen as a long standing fund where friends / stakeholders can create and vote on proposals which democractically decides where funds can be redirected.
    - A private pool has a list of address that are allowed to deposit. The owner of the fund has to add the address to the list in order for the depositor to deposit. This can be a way to integrate KYC/AML.
    - A public pool allows anyone to deposit.

- ETF
  - Coming soon..

## Deployment

### Build

For development cargo build can be used:

```sh
cargo build --features program
```

To build the smart-contract for deployment it must be built for the solana BPF target.

> Note: install build-bpf is required. Instructions can be found in the Solana documentation, [here](https://docs.solana.com/cli/install-solana-cli-tools)

BPF Target:

```sh
cargo build-bpf --features program
```

## Command Line

~~Not fully implemented~~

```sh
cd cli && cargo install --path .
```

The squifi cli is designed to help users to create funds, check balances, whitelist accounts, deposit and withdraw funds, make and vote on proposals and to initialize paybacks.

## Features

- [x] Create funding pool
- [x] Add tokens to Funding pool
- [x] Owner of pool withdraw tokens
- [x] Create a token to represent pool ownership
- [ ] Ability to create proposals to withdraw money
- [ ] Vote on proposals
- [ ] Add pool contributor paybacks
- [ ] Rage quit by depositors
