//! 运动指令集 `trait`
//!
//! ## 轨迹运动
//! * 关节运动 `movej`
//!

use serde::{Deserialize, Serialize};

use crate::{ArmType, RStepType, Result, StepType, TrajectoryConnect};

use super::{
    ArmContinueResponse, ArmPauseResponse, ArmSlowStopResponse, ArmStopResponse,
    ReceiveStateResponse,
};

/// 运动指令接收结果
#[derive(Debug, Serialize, Deserialize)]
pub struct MotionData {
    /// 指令名称
    command: String,
    /// 接收状态结果
    receive_state: bool,
}

pub trait MotionTrait {
    /// 关节运动
    ///
    /// * `arm_type` 手臂目标关节角度，精度 0.001°。
    /// * `v` 速度百分比例系数，0~100。
    /// * `r` 交融半径百分比系数，0-100。
    /// * `trajectory_connect` 是否和下一条运动一起规划，0 代表立即规划，1 代表和下一条轨迹一起规划，当为 1 时，轨迹不会立即执行。
    fn movej(
        &mut self,
        arm_type: ArmType,
        v: u8,
        r: u8,
        trajectory_connect: TrajectoryConnect,
    ) -> Result<MotionData>;

    /// 关节步进
    /// 控制机械臂某个关节的步进运动。
    ///
    /// * `joint_step` （1）步进关节号；（2）关节步进角度，单位：°，精度：0.001°。
    /// * `v` 速度百分比例系数，0~10。
    fn set_joint_step(&mut self, joint_step: &[i32; 2], v: u8) -> Result<ReceiveStateResponse>;

    /// 位置步进
    /// 控制机械臂沿 x、y、z 轴方向直线步进运动。
    ///
    /// * `step_type` 步进类型，x_step 为 X 轴方向，y_step 为 Y 轴方向，z_step 为 Z 轴方向。
    /// * `step` 步进距离单位：m，精度：0.001mm，即 0.000001m。
    /// * `v` 速度百分比例系数，0~100。
    fn set_pos_step(
        &mut self,
        step_type: StepType,
        step: isize,
        v: u8,
    ) -> Result<ReceiveStateResponse>;

    /// 姿态步进
    /// 控制机械臂沿 x、y、z 轴方向旋转步进运动。
    ///
    /// * `step_type` 步进方向，rx_step：绕 X 轴旋转，ry_step：绕 Y 旋转，rz_step：绕 Z 轴旋转。
    /// * `step` 步进弧度，单位：rad，精度 0.001rad。
    /// * `v` 速度百分比例系数，0~100。
    fn set_ort_step(
        &mut self,
        step_type: RStepType,
        step: isize,
        v: u8,
    ) -> Result<ReceiveStateResponse>;

    /// 轨迹急停
    fn set_arm_stop(&mut self) -> Result<ArmStopResponse>;

    /// 轨迹缓停
    /// 在当前正在运行的轨迹上停止。
    fn set_arm_slow_stop(&mut self) -> Result<ArmSlowStopResponse>;

    /// 轨迹暂停
    /// 停在轨迹上，轨迹可恢复。
    fn set_arm_pause(&mut self) -> Result<ArmPauseResponse>;

    /// 轨迹暂停后恢复
    fn set_arm_continue(&mut self) -> Result<ArmContinueResponse>;
}
