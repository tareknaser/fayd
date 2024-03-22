use crate::{requests::Send, AppState};
use actix_web::{error, web, Error, HttpResponse};
use paperclip::actix::{api_v2_operation, get, post, web::Json};

/// Endpoint for getting the Faucet balance
#[api_v2_operation]
#[get("/balance")]
pub async fn get_balance(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let result = faucet.confirmed_balance();
    let val = serde_json::to_value(result)
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;
    Ok(HttpResponse::Ok().json(val))
}

/// Endpoint for syncing the Faucet
#[api_v2_operation]
#[post("/sync")]
pub async fn sync_faucet(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let result = faucet.sync();
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Faucet synced")),
        Err(err) => Err(error::ErrorInternalServerError(format!("Error: {}", err))),
    }
}

/// Endpoint for sending funds to an address
#[api_v2_operation]
#[post("/send")]
pub async fn send_funds(
    data: web::Data<AppState>,
    body: Json<Send>,
) -> Result<HttpResponse, Error> {
    let mut faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let address = body.address.clone();
    let result = faucet.send(&address);
    match result {
        Ok(txid) => Ok(HttpResponse::Ok().body(format!("Sent funds to {}", txid))),
        Err(err) => Err(error::ErrorInternalServerError(format!("Error: {}", err))),
    }
}

/// Endpoint for getting a deposit address
#[api_v2_operation]
#[get("/deposit")]
pub async fn get_deposit_address(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let address = faucet.deposit();
    match address {
        Ok(address) => Ok(HttpResponse::Ok().body(address)),
        Err(err) => Err(error::ErrorInternalServerError(format!("Error: {}", err))),
    }
}
