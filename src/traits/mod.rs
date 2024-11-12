// 运动指令集
mod motion;
pub use motion::*;

// modbus指令集
mod modbus;
pub use modbus::*;

// IO配置指令集
mod io;
pub use io::*;
use serde::{Deserialize, Serialize};

// 统一返回设置数据
#[derive(Debug, Serialize, Deserialize)]
pub struct SetStateResponse {
    command: String,
    set_state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteStateResponse {
    command: String,
    write_state: bool,
}
