//! 睿尔曼机械臂
//!
//! 用于初始化、连接和控制机械臂
//!

use std::{
    net::{SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
};

use crate::{config, Result, Transport};

#[derive(Debug)]
pub struct Realman {
    ip: String,
    port: u16,
}

impl Realman {
    /// 创建新的机械臂实例
    ///
    /// * `ip` 机械臂的IP地址
    /// * `port` 机械臂端口号
    pub fn new(ip: impl Into<String>, port: u16) -> Realman {
        Realman {
            ip: ip.into(),
            port,
        }
    }

    /// 连接机械臂
    pub fn connect(&self) -> Result<Transport> {
        let socket = TcpStream::connect_timeout(
            &SocketAddr::from_str(format!("{}:{}", self.ip, self.port).as_str())?,
            Duration::from_secs(5),
        )?;

        Ok(Transport::new(socket))
    }
}

impl Default for Realman {
    fn default() -> Self {
        Self::new(config::DEFAULT_IP, config::DEFUALT_PORT)
    }
}
