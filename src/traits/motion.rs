//! 运动指令集

use serde::{Deserialize, Serialize};

use crate::{ArmType, Result, TrajectoryConnect};

#[derive(Debug, Serialize, Deserialize)]
pub struct MotionData {
    command: String,
    receive_state: bool,
}

pub trait MotionTrait {
    /// 关节运动
    ///
    /// * arm_type 手臂目标关节角度，精度 0.001°。
    /// * v 速度百分比例系数，0~100。
    /// * r 交融半径百分比系数，0-100。
    /// * trajectory_connect 可选参数，代表是否和下一条运动一起规划，0 代表立即规划，1 代表和下一条轨迹一起规划，当为 1 时，轨迹不会立即执行。
    fn movej(
        &mut self,
        arm_type: ArmType,
        v: u8,
        r: u8,
        trajectory_connect: TrajectoryConnect,
    ) -> Result<MotionData>;
}