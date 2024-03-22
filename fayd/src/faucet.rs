use anyhow::{anyhow, Result};
use std::{str::FromStr, sync::mpsc::sync_channel, thread::spawn};

use bdk::{
    bitcoin::{Address, Block, Network, Transaction, Txid},
    wallet::{ChangeSet, Wallet},
    SignOptions,
};
use bdk_bitcoind_rpc::{
    bitcoincore_rpc::{Client, RpcApi},
    Emitter,
};
use bdk_file_store::Store;

use crate::args::FaucetArgs;

#[derive(Debug)]
enum Emission {
    Block(bdk_bitcoind_rpc::BlockEvent<Block>),
    Mempool(Vec<(Transaction, u64)>),
}

pub struct Faucet {
    args: FaucetArgs,
    wallet: Wallet<Store<ChangeSet>>,
    client: Client,
}

impl Faucet {
    /// Create a new faucet
    pub fn new(args: FaucetArgs) -> Result<Self> {
        let wallet = args.wallet()?;
        let client = args.client()?;

        Ok(Self {
            args,
            wallet,
            client,
        })
    }

    /// Return the balance of the faucet
    pub fn confirmed_balance(&self) -> u64 {
        self.wallet.get_balance().confirmed
    }

    /// Sync the faucet
    pub fn sync(&mut self) -> Result<()> {
        let wallet_tip = self.wallet.latest_checkpoint();
        let emitter_tip = wallet_tip.clone();
        let (sender, receiver) = sync_channel::<Emission>(21);

        let client = self.args.client()?;
        let start_height = self.wallet.latest_checkpoint().height();
        spawn(move || -> Result<()> {
            let mut emitter = Emitter::new(&client, emitter_tip, start_height);
            while let Some(emission) = emitter.next_block()? {
                sender.send(Emission::Block(emission))?;
            }
            sender.send(Emission::Mempool(emitter.mempool()?))?;
            Ok(())
        });

        for emission in receiver {
            match emission {
                Emission::Block(block_emission) => {
                    let height = block_emission.block_height();
                    let connected_to = block_emission.connected_to();
                    self.wallet.apply_block_connected_to(
                        &block_emission.block,
                        height,
                        connected_to,
                    )?;
                    self.wallet.commit()?;
                }
                Emission::Mempool(mempool_emission) => {
                    self.wallet.apply_unconfirmed_txs(
                        mempool_emission.iter().map(|(tx, time)| (tx, *time)),
                    );
                    self.wallet.commit()?;
                    break;
                }
            }
        }
        Ok(())
    }

    /// Send funds to an address
    pub fn send(&mut self, address: &str) -> Result<Txid> {
        let wallet_balance = self.confirmed_balance();
        if wallet_balance < self.args.amount() {
            return Err(anyhow!("Insufficient funds"));
        }

        let address =
            Address::from_str(address).map_err(|e| anyhow!("Failed to parse address: {}", e))?;
        let address = address
            .require_network(Network::Signet)
            .map_err(|e| anyhow!("Address is not on the Signet network: {}", e))?;

        let mut tx_builder = self.wallet.build_tx();
        tx_builder
            .add_recipient(address.script_pubkey(), self.args.amount())
            .enable_rbf();

        let mut psbt = tx_builder
            .finish()
            .map_err(|e| anyhow!("Failed to build transaction: {}", e))?;
        let finalized = self
            .wallet
            .sign(&mut psbt, SignOptions::default())
            .map_err(|e| anyhow!("Failed to sign transaction: {}", e))?;
        if !finalized {
            return Err(anyhow!("Failed to finalize transaction"));
        }

        let tx = psbt.extract_tx();
        self.client
            .send_raw_transaction(&tx)
            .map_err(|e| anyhow!("Failed to broadcast transaction: {}", e))?;

        Ok(tx.txid())
    }

    /// Deposit funds to the faucet
    ///
    /// Returns the address to send funds to
    pub fn deposit(&mut self) -> Result<String> {
        let address = self
            .wallet
            .try_get_address(bdk::wallet::AddressIndex::New)
            .map_err(|e| anyhow!("Failed to get new address from wallet: {}", e))?;

        Ok(address.to_string())
    }
}
