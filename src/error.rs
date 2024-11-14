//! 错误信息Error

use std::{io, net};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// `IP`地址解析错误
    #[error("robot addr parse error")]
    AddrParse(#[from] net::AddrParseError),
    /// 连接机械臂错误
    #[error("robot disconnected")]
    Disconnect(#[from] io::Error),
    /// 数据转换错误
    #[error("json convert error")]
    Json(#[from] serde_json::Error),
}
