use serde::{Deserialize, Serialize};

// 运动指令集
mod motion;
/// 运动指令集
pub use motion::*;

// modbus指令集
mod modbus;
/// modbus指令集
pub use modbus::*;

// IO配置指令集
mod io;
/// IO配置指令集
pub use io::*;

// 状态查询指令集
mod arm_state;
/// 状态查询指令集
pub use arm_state::*;

/// 统一返回设置数据
#[derive(Debug, Serialize, Deserialize)]
pub struct SetStateResponse {
    pub command: String,
    pub set_state: bool,
}

/// 写单个寄存器结果
#[derive(Debug, Serialize, Deserialize)]
pub struct WriteStateResponse {
    pub command: String,
    pub write_state: bool,
}

/// 直线运动指令接收结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveStateResponse {
    /// 直线运动命令名称
    pub command: String,
    /// 接收结果
    pub receive_state: bool,
}

/// 轨迹急停返回结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ArmStopResponse {
    pub command: String,
    pub arm_stop: bool,
}
/// 轨迹缓停返回结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ArmSlowStopResponse {
    pub command: String,
    pub arm_slow_stop: bool,
}
/// 轨迹暂停返回结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ArmPauseResponse {
    pub command: String,
    pub arm_pause: bool,
}
/// 轨迹暂停后恢复返回结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ArmContinueResponse {
    pub command: String,
    pub arm_continue: bool,
}
