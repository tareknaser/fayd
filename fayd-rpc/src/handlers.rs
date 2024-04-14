use crate::AppState;
use actix_web::{error, web, Error, HttpResponse};
use paperclip::actix::{api_v2_operation, get, post};

/// Endpoint for getting the Faucet balance
#[api_v2_operation]
#[get("/balance")]
pub async fn get_balance(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("/balance endpoint called");
    let faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let result = faucet.confirmed_balance();
    let response = serde_json::json!({
        "status": "success",
        "confirmed_balance_msat": result,
    });
    Ok(HttpResponse::Ok().json(response))
}

/// Endpoint for syncing the Faucet
#[api_v2_operation]
#[post("/sync")]
pub async fn sync_faucet(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("/sync endpoint called");
    let mut faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let result = faucet.sync();
    match result {
        Ok(_) => {
            let response = serde_json::json!({
                "status": "success",
                "message": "Faucet synced successfully",
            });
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => Err(error::ErrorInternalServerError(format!("Error: {}", err))),
    }
}

/// Endpoint for sending funds to an address
#[api_v2_operation]
#[post("/send")]
pub async fn send_funds(data: web::Data<AppState>, body: String) -> Result<HttpResponse, Error> {
    let address = body;
    log::info!("/send endpoint called to send funds to {:?}", address);
    let mut faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let address = address.clone();
    let result = faucet.send(&address);
    match result {
        Ok(txid) => {
            let response = serde_json::json!({
                "status": "success",
                "txid": txid,
            });
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => Err(error::ErrorInternalServerError(format!("Error: {}", err))),
    }
}

/// Endpoint for getting a deposit address
#[api_v2_operation]
#[get("/deposit")]
pub async fn get_deposit_address(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("/deposit endpoint called");
    let mut faucet = data
        .faucet
        .lock()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("Error: {}", err)))?;

    let address = faucet.deposit();
    match address {
        Ok(address) => {
            let response = serde_json::json!({
                "status": "success",
                "address": address,
            });
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => Err(error::ErrorInternalServerError(format!("Error: {}", err))),
    }
}
