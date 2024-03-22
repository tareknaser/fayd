use actix_web::{App, HttpServer};
use anyhow::Result;
use clap::Parser;
use fayd::{Faucet, FaucetArgs};
use paperclip::actix::{web, OpenApiExt};
use std::sync::{Arc, Mutex};

use crate::handlers::{get_balance, get_deposit_address, send_funds, sync_faucet};

mod handlers;
mod requests;

#[derive(Debug, Clone, Parser)]
#[clap(about = "Fayd is a bitcoin signet faucet")]
struct FaucetRpcArgs {
    #[clap(flatten)]
    faucet_args: FaucetArgs,
    #[clap(short, long, default_value = "8080", help = "Port to listen on")]
    port: u16,
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Clone, Parser)]
enum Cmd {
    #[clap(about = "Run the faucet server")]
    Run,
}

struct AppState {
    faucet: Arc<Mutex<Faucet>>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = FaucetRpcArgs::parse();
    let faucet = Faucet::new(args.faucet_args)?;
    let faucet = Arc::new(Mutex::new(faucet));

    let host = "127.0.0.1";
    let port = args.port;

    HttpServer::new(move || {
        let state = AppState {
            faucet: faucet.clone(),
        };
        App::new()
            .app_data(web::Data::new(state))
            .wrap_api()
            .service(get_balance)
            .service(sync_faucet)
            .service(send_funds)
            .service(get_deposit_address)
            .with_json_spec_at("/api/v1")
            .build()
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
