use anyhow::Result;
use clap::{self, Parser};
use std::path::PathBuf;

use bdk::{
    bitcoin::Network,
    wallet::{ChangeSet, Wallet},
};
use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client};
use bdk_file_store::Store;

const DB_MAGIC: &str = "fayd";

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum FaucetNetwork {
    #[clap(name = "testnet")]
    Testnet,
    #[clap(name = "signet")]
    Signet,
    #[clap(name = "regtest")]
    Regtest,
}

impl From<FaucetNetwork> for Network {
    fn from(network: FaucetNetwork) -> Self {
        match network {
            FaucetNetwork::Testnet => Network::Testnet,
            FaucetNetwork::Signet => Network::Signet,
            FaucetNetwork::Regtest => Network::Regtest,
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[clap(about = "Fayd is a bitcoin signet faucet")]
pub struct FaucetArgs {
    /// Wallet descriptor
    #[clap(env = "DESCRIPTOR", long, help = "Wallet descriptor")]
    pub descriptor: String,
    /// Where to store wallet data
    #[clap(long, default_value = ".fayd.db", help = "Path to the wallet database")]
    pub db_path: PathBuf,

    /// Network to use
    #[clap(long, default_value = "signet", help = "Network to use")]
    pub network: FaucetNetwork,
    /// RPC URL
    #[clap(
        env = "RPC_URL",
        long,
        default_value = "127.0.0.1:8332",
        help = "Bitcoin Core RPC URL"
    )]
    pub url: String,
    /// RPC auth cookie file
    #[clap(env = "RPC_COOKIE", long, help = "Bitcoin Core RPC cookie file")]
    pub rpc_cookie: Option<PathBuf>,
    /// RPC auth username
    #[clap(env = "RPC_USER", long, help = "Bitcoin Core RPC username")]
    pub rpc_user: Option<String>,
    /// RPC auth password
    #[clap(env = "RPC_PASS", long, help = "Bitcoin Core RPC password")]
    pub rpc_pass: Option<String>,

    /// Amount to send to each address
    #[clap(
        long,
        default_value = "100000",
        help = "Amount to send to each address"
    )]
    pub amount: u64,
}

impl FaucetArgs {
    pub(crate) fn amount(&self) -> u64 {
        self.amount
    }

    pub(crate) fn client(&self) -> Result<Client> {
        Ok(Client::new(
            &self.url,
            match (&self.rpc_cookie, &self.rpc_user, &self.rpc_pass) {
                (None, None, None) => Auth::None,
                (Some(path), _, _) => Auth::CookieFile(path.clone()),
                (_, Some(user), Some(pass)) => Auth::UserPass(user.clone(), pass.clone()),
                (_, Some(_), None) => panic!("rpc auth: missing rpc_pass"),
                (_, None, Some(_)) => panic!("rpc auth: missing rpc_user"),
            },
        )?)
    }

    pub(crate) fn wallet(&self) -> Result<Wallet<Store<ChangeSet>>> {
        let db = Store::<ChangeSet>::open_or_create_new(DB_MAGIC.as_bytes(), self.db_path.clone())?;
        let network = Network::from(self.network.clone());
        let wallet = Wallet::new_or_load(self.descriptor.as_str(), None, db, network)?;
        Ok(wallet)
    }
}
