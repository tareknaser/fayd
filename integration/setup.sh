#!/bin/sh
set -e

sudo apt-get install build-essential
sudo apt-get install libtool autotools-dev autoconf
sudo apt-get install libssl-dev
sudo apt-get install libboost-all-dev
sudo add-apt-repository ppa:luke-jr/bitcoincore
sudo apt-get update
sudo apt-get install jq
sudo apt-get install bitcoind

mkdir ~/.bitcoin/
cat <<EOF > ~/.bitcoin/bitcoin.conf
rpcuser=foo
rpcpassword=bar
fallbackfee=0.00001
EOF

bitcoind -regtest -daemon
sleep 5
bitcoin-cli -regtest createwallet test
