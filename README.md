# Fayd

Fayd ( **فَيْض** - meaning "abundance" in arabic) is a Bitcoin Signet faucet built on top of `BDK`'s `bitcoind_rpc` crate talking to a local Signet `bitcoind` node. It allows users to request and receive signet coins for testing purposes.

## Installation

1. Clone the repository: `git clone https://github.com/tareknaser/fayd.git`
2. Navigate to the core crate directory: `cd fayd/fayd-rpc`
3. Install the tool: `cargo install --force --locked --path .`

## Usage

**To start the server, use the following command:**

```bash
fayd-rpc --rpc-user <RPC_USER> --rpc-pass <RPC_PASS> --descriptor <DESCRIPTOR> run
```

**Get Balance**

```bash
curl http://127.0.0.1:8080/balance
```

**Sync Faucet**

```bash
curl -X POST http://127.0.0.1:8080/sync
```

**Get a Deposit Address**

```bash
curl http://127.0.0.1:8080/deposit
```

**Send Funds to Address**

```bash
curl -X POST --data <address> http://127.0.0.1:8080/send
```

### Options

```bash
Fayd is a bitcoin signet faucet

Usage: fayd-rpc [OPTIONS] --descriptor <DESCRIPTOR> <COMMAND>

Commands:
  run   Run the faucet server
  help  Print this message or the help of the given subcommand(s)

Options:
      --descriptor <DESCRIPTOR>  Wallet descriptor [env: DESCRIPTOR=]
      --db-path <DB_PATH>        Path to the wallet database [default: .fayd.db]
      --url <URL>                Bitcoin Core RPC URL [env: RPC_URL=] [default: 127.0.0.1:8332]
      --rpc-cookie <RPC_COOKIE>  Bitcoin Core RPC cookie file [env: RPC_COOKIE=]
      --rpc-user <RPC_USER>      Bitcoin Core RPC username [env: RPC_USER=]
      --rpc-pass <RPC_PASS>      Bitcoin Core RPC password [env: RPC_PASS=]
      --amount <AMOUNT>          Amount to send to each address [default: 100000]
  -p, --port <PORT>              Port to listen on [default: 8080]
  -h, --help                     Print help

```

## Packages

| Package    | Description                                                       | Version     |
| ---------- | ----------------------------------------------------------------- | ----------- |
| `fayd`     | Core crate defining the `Faucet` wallet struct                    | pre-release |
| `fayd-rpc` | REST API crate for interacting with `fayd` and running the faucet | pre-release |
