//! 状态查询指令集 `trait`
//!
//! ## 机械臂状态
//! * 查询机械臂状态 `get_current_arm_state`
//!

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmStateData {
    pub state: String,
    pub arm_state: ArmState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmState {
    pub joint: Vec<i32>,
    pub pose: Vec<i32>,
    pub arm_err: i32,
    pub sys_err: i32,
}

pub trait ArmStateTrait {
    /// 查询机械臂状态
    ///
    /// 反馈机械臂状态，信息如下：
    ///
    /// 六自由度机械臂关节1~6角度依次为：0.1°，0.2°，0.3°。0.4°，0.5°，0.6°，精度：0.001°；
    ///
    /// 七自由度机械臂关节1~7角度依次为：0.1°，0.2°，0.3°。0.4°，0.5°，0.6°，0.7°，精度：0.001°；
    ///
    /// 位置：x：0.1m，y:0.2m，z：0.03m，位置精度：0.001mm；
    ///
    /// 姿态：rx：0.4rad，ry：0.5rad，rz：0.6rsd，姿态精度：0.001rad；
    ///
    /// arm_err：机械臂错误代码，指机械臂运行规划中的软件错误：0；
    ///
    /// sys_err：控制器错误代码，指控制器运行过程中的硬件错误：0。
    fn get_current_arm_state(&mut self) -> Result<ArmStateData>;
}
