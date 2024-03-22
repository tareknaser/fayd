use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Apiv2Schema, Serialize)]
pub struct Send {
    pub address: String,
}
