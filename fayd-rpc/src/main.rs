use crate::handlers::{get_balance, get_deposit_address, send_funds, sync_faucet};
use actix_web::{App, HttpServer};
use anyhow::Result;
use clap::Parser;
use fayd::{Faucet, FaucetArgs};
use paperclip::actix::{web, OpenApiExt};
use std::sync::{Arc, Mutex};

mod handlers;
mod requests;

struct AppState {
    faucet: Arc<Mutex<Faucet>>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = FaucetArgs::parse();
    let faucet = Faucet::new(args)?;
    let faucet = Arc::new(Mutex::new(faucet));

    let host = "127.0.0.1";
    let port = 8080;

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
