use serde::{Deserialize, Serialize};

// 运动指令集
mod motion;
pub use motion::*;

// modbus指令集
mod modbus;
pub use modbus::*;

// IO配置指令集
mod io;
pub use io::*;

// 状态查询指令集
mod arm_state;
pub use arm_state::*;

// 统一返回设置数据
#[derive(Debug, Serialize, Deserialize)]
pub struct SetStateResponse {
    pub command: String,
    pub set_state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteStateResponse {
    pub command: String,
    pub write_state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveStateResponse {
    pub command: String,
    pub receive_state: bool,
}
