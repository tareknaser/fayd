#!/bin/bash
set -e


# This is a simple shell script to test fayd-rpc
# It assumes bitcoind is running on 127.0.0.1:18443 on regtest
#
# Script steps:
# - Run fayd-rpc in the background
# - Sync and get balance (should be 0)
# - Get a deposit address
# - Generate 25 blocks to the deposit address
# - Generate 100 blocks
# - Sync and get balance (should be 25)
# - Send 10 BTC to a regtest address 
# - Get the transaction details using the txid 
# - Generate 100 blocks
# - Sync and get balance (should be 15)
#
# Note: Run this script from the root of the repository

home=$(pwd)
# Add fayd-rpc to PATH (assumes `cargo build --release` has been run)
export PATH=$home/target/release:$PATH

# Run fayd-rpc in the background
descriptor="wpkh(tprv8ZgxMBicQKsPeygpVjnnQgW7TNnZZ6xPaEhGGUTnh5uSPGbJ2eRBtiC8oE41LmgiUHe4W5ec6crG8DSVjhBt1PQaMTx43Ffi5DwgBQiKf5o/84'/1'/0'/1/*)#fn34tvwr"
fayd-rpc --url 127.0.0.1:18443 --rpc-user foo --rpc-pass bar --network regtest --descriptor "$descriptor" run &
sleep 5

# Sync
sync_result=$(curl -X POST http://127.0.0.1:8080/sync)

# Get Balance
balance_result=$(curl http://127.0.0.1:8080/balance)
intial_balance=$(echo $balance_result | jq '.confirmed_balance_msat')

# Get a deposit address
deposit_result=$(curl http://127.0.0.1:8080/deposit)
deposit_address=$(echo $deposit_result | jq '.address')

# remove quotes
deposit_address=$(echo $deposit_address | tr -d '"')

# Generate 25 blocks to the deposit address
bitcoin-cli -regtest generatetoaddress 25 $deposit_address

# Generate 100 blocks
bitcoin-cli -regtest -generate 100
sleep 5

# Sync
sync_result=$(curl -X POST http://127.0.0.1:8080/sync)

# Get Balance
balance_result=$(curl http://127.0.0.1:8080/balance)
balance=$(echo $balance_result | jq '.confirmed_balance_msat')
current_balance=intial_balance+25
if [ $balance -ne $current_balance ]; then
    echo "Balance is not 25"
    echo $balance
    exit 1
fi

# Send BTC to an address
send_address="bcrt1qr8tlsvn6armmnrtndslqelwx5ycs8uf5kzfk8w"
send_result=$(curl -X POST --data "$send_address" http://127.0.0.1:8080/send)
echo $send_result
txid=$(echo $send_result | jq '.txid')
txid=$(echo $txid | tr -d '"')

# Get the transaction details
tx_details=$(bitcoin-cli -regtest getrawtransaction $txid 1)
send_success=false
for i in $(seq 0 $(echo $tx_details | jq '.vout | length')); do
    address=$(echo $tx_details | jq ".vout[$i].scriptPubKey.address")
    address=$(echo $address | tr -d '"')
    if [ $address == $send_address ]; then
        send_success=true
    fi
done
if [ $send_success == false ]; then
    echo "Send transaction failed"
    echo $tx_details
    exit 1
fi

echo "Fayd-rpc sanity check passed"
