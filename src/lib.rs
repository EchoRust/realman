//! # 睿尔曼机械臂 Rust SDK
//!
//! [![Crates.io][crates-badge]][crates-url]
//! [![Docs.rs][doc-badge]][doc-url]
//! [![MIT licensed][mit-badge]][mit-url]
//!
//! [crates-badge]: https://img.shields.io/badge/crates-0.2.0-yellow
//! [crates-url]: https://crates.io/crates/realman
//! [doc-badge]: https://img.shields.io/badge/doc-latest-blue
//! [doc-url]: https://docs.rs/realman
//! [mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
//! [mit-url]: https://opensource.org/licenses/MIT
//!
//! 这是睿尔曼机械臂的 `Rust SDK` ，旨在帮助 `Rust` 开发者更好的对睿尔曼机械臂进行二次开发。
//!
//! `SDK` 是根据官网 `json` 协议开发。[官网链接](https://develop.realman-robotics.com/robot/json/getStartedJson.html)
//!
//! ## 作者介绍
//!
//! * 作者: 李扬(Leon)
//! * 个人网站: 技安Rust笔记 - [https://echoli.cn](https://echoli.cn)
//! * Github: [https://github.com/EchoRust/realman](https://github.com/EchoRust/realman)
//!
//! ## 使用方法
//!
//! ```toml
//! [dependencies]
//! realman = "0.2.0"
//! ```
//!
//! ## 三次点头示例
//!
//! ```rust
//! use realman::{traits::MotionTrait, ArmType, Realman, TrajectoryConnect};
//!
//! let realman = Realman::default();
//!
//! if let Ok(mut socket) = realman.connect() {
//!     socket.movej(ArmType::Gen72([0; 7]), 50, 0, TrajectoryConnect::Now).ok();
//!
//!     for _ in 0..3 {
//!         socket.set_joint_step(&[6, -15000], 50).ok();
//!         socket.set_joint_step(&[6, 15000], 50).ok();
//!     }
//! }
//! ```

mod config;

// 错误处理
mod error;
/// 错误信息
pub use error::Error;

/// 指令集 Trait
pub mod traits;

// 数据类型
mod types;
pub use types::*;

mod robot;

// 睿尔曼
mod realman;
/// 睿尔曼机械臂
pub use realman::Realman;

// Transport
mod transport;
/// 机械臂传输
pub use transport::Transport;

/// 结果类型
pub type Result<T> = std::result::Result<T, error::Error>;
