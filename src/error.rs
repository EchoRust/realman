use std::{io, net};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("robot addr parse error")]
    AddrParse(#[from] net::AddrParseError),
    #[error("robot disconnected")]
    Disconnect(#[from] io::Error),
    #[error("json convert error")]
    Json(#[from] serde_json::Error),
}
