//! # 睿尔曼机械臂 Rust SDK
//!
//! 这是睿尔曼机械臂的 `Rust SDK` ，旨在帮助 `Rust` 开发者更好的对睿尔曼机械臂进行二次开发。
//!
//! * 作者：Leon 李扬
//! * 个人网站：[技安Rust笔记](https://echoli.cn)
//!

mod config;

// 错误处理
mod error;

// trait
pub mod traits;

// 数据类型
mod types;
pub use types::*;

// 运动指令集
mod motion;

// modbus指令集
mod modbus;

// IO指令集
mod io;

// 睿尔曼
mod realman;
pub use realman::Realman;

// Transport
mod transport;
pub use transport::Transport;

pub type Result<T> = std::result::Result<T, error::Error>;
