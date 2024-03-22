use anyhow::Result;
use clap::{self, Args};
use std::path::PathBuf;

use bdk::{
    bitcoin::Network,
    wallet::{ChangeSet, Wallet},
};
use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client};
use bdk_file_store::Store;

const DB_MAGIC: &str = "fayd";

#[derive(Args, Debug, Clone)]
#[clap(about = "Arguments for the faucet")]
pub struct FaucetArgs {
    /// Wallet descriptor
    #[clap(env = "DESCRIPTOR")]
    pub descriptor: String,
    /// Where to store wallet data
    #[clap(long, default_value = ".fayd.db")]
    pub db_path: PathBuf,

    /// RPC URL
    #[clap(env = "RPC_URL", long, default_value = "127.0.0.1:8332")]
    pub url: String,
    /// RPC auth cookie file
    #[clap(env = "RPC_COOKIE", long)]
    pub rpc_cookie: Option<PathBuf>,
    /// RPC auth username
    #[clap(env = "RPC_USER", long)]
    pub rpc_user: Option<String>,
    /// RPC auth password
    #[clap(env = "RPC_PASS", long)]
    pub rpc_pass: Option<String>,

    /// Amount to send to each address
    #[clap(long, default_value = "100000")]
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
        let wallet = Wallet::new_or_load(self.descriptor.as_str(), None, db, Network::Signet)?;
        Ok(wallet)
    }
}
