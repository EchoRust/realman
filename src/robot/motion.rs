//! 运动指令集

use std::io::{Read, Write};

use log::info;
use serde::Serialize;

use crate::{
    traits::{
        ArmContinueResponse, ArmJointTeachResponse, ArmOrtTeachResponse, ArmPauseResponse,
        ArmPosTeachResponse, ArmSlowStopResponse, ArmStopResponse, ArmStopTeachResponse,
        MotionData, MotionTrait, ReceiveStateResponse,
    },
    ArmType, DirectionType, Result, StepType, TeachRotateType, TeachType, TrajectoryConnect,
    Transport,
};

impl MotionTrait for Transport {
    fn movej(
        &mut self,
        arm_type: ArmType,
        v: u8,
        r: u8,
        trajectory_connect: TrajectoryConnect,
    ) -> Result<MotionData> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            joint: Vec<i32>,
            v: u8,
            r: u8,
            trajectory_connect: u8,
        }

        let cmd = Command {
            command: "movej",
            joint: arm_type.into(),
            v: if v > 100 { 100 } else { v },
            r: if r > 100 { 100 } else { r },
            trajectory_connect: trajectory_connect.into(),
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp = serde_json::from_str::<MotionData>(&String::from_utf8_lossy(&buf[0..n]))?;

        if trajectory_connect == TrajectoryConnect::Now {
            // 等待机械臂达到终点
            if let Ok(n) = self.read(&mut buf) {
                info!("movej result: {}", String::from_utf8_lossy(&buf[0..n]));
            }
        }

        Ok(cmd_resp)
    }

    /// 位置步进
    /// 控制机械臂沿 x、y、z 轴方向直线步进运动。
    fn set_pos_step(
        &mut self,
        step_type: StepType,
        step: isize,
        v: u8,
    ) -> Result<ReceiveStateResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            step_type: String,
            v: u8,
            step: isize,
        }

        let cmd = Command {
            command: "set_pos_step",
            step_type: step_type.to_string(),
            v: if v > 100 { 100 } else { v },
            step,
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ReceiveStateResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        if cmd_resp.receive_state {
            // 等待机械臂达到终点
            if let Ok(n) = self.read(&mut buf) {
                info!(
                    "set_pos_step result: {}",
                    String::from_utf8_lossy(&buf[0..n])
                );
            }
        }

        Ok(cmd_resp)
    }

    /// 关节步进
    /// 控制机械臂某个关节的步进运动。
    fn set_joint_step(&mut self, joint_step: &[i32; 2], v: u8) -> Result<ReceiveStateResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            joint_step: Vec<i32>,
            v: u8,
        }

        let cmd = Command {
            command: "set_joint_step",
            joint_step: joint_step.to_vec(),
            v: if v > 10 { 10 } else { v },
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ReceiveStateResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        if cmd_resp.receive_state {
            // 等待机械臂达到终点
            if let Ok(n) = self.read(&mut buf) {
                info!(
                    "set_joint_step result: {}",
                    String::from_utf8_lossy(&buf[0..n])
                );
            }
        }

        Ok(cmd_resp)
    }

    /// 姿态步进
    /// 控制机械臂沿 x、y、z 轴方向旋转步进运动。
    fn set_ort_step(
        &mut self,
        step_type: crate::RStepType,
        step: isize,
        v: u8,
    ) -> Result<ReceiveStateResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            step_type: String,
            v: u8,
            step: isize,
        }

        let cmd = Command {
            command: "set_ort_step",
            step_type: step_type.to_string(),
            v: if v > 100 { 100 } else { v },
            step,
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ReceiveStateResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        if cmd_resp.receive_state {
            // 等待机械臂达到终点
            if let Ok(n) = self.read(&mut buf) {
                info!(
                    "set_ort_step result: {}",
                    String::from_utf8_lossy(&buf[0..n])
                );
            }
        }

        Ok(cmd_resp)
    }

    /// 轨迹急停
    fn set_arm_stop(&mut self) -> Result<ArmStopResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
        }

        let cmd = Command {
            command: "set_arm_stop",
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmStopResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    /// 轨迹缓停
    /// 在当前正在运行的轨迹上停止。
    fn set_arm_slow_stop(&mut self) -> Result<ArmSlowStopResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
        }

        let cmd = Command {
            command: "set_arm_slow_stop",
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmSlowStopResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    /// 轨迹暂停
    /// 停在轨迹上，轨迹可恢复。
    fn set_arm_pause(&mut self) -> Result<ArmPauseResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
        }

        let cmd = Command {
            command: "set_arm_pause",
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmPauseResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    /// 轨迹暂停后恢复
    fn set_arm_continue(&mut self) -> Result<ArmContinueResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
        }

        let cmd = Command {
            command: "set_arm_continue",
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmContinueResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    /// 关节示教
    fn set_joint_teach(
        &mut self,
        teach_joint: u8,
        direction: DirectionType,
        v: u8,
    ) -> Result<ArmJointTeachResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            teach_joint: u8,
            direction: String,
            v: u8,
        }

        let cmd = Command {
            command: "set_joint_teach",
            teach_joint,
            direction: direction.to_string(),
            v: if v > 100 { 100 } else { v },
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmJointTeachResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    /// 位置示教
    fn set_pos_teach(
        &mut self,
        teach_type: TeachType,
        direction: DirectionType,
        v: u8,
    ) -> Result<ArmPosTeachResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            teach_type: String,
            direction: String,
            v: u8,
        }

        let cmd = Command {
            command: "set_pos_teach",
            teach_type: teach_type.to_string(),
            direction: direction.to_string(),
            v: if v > 100 { 100 } else { v },
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmPosTeachResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    /// 姿态示教
    fn set_ort_teach(
        &mut self,
        teach_rotate_joint: TeachRotateType,
        direction: DirectionType,
        v: u8,
    ) -> Result<ArmOrtTeachResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            teach_type: String,
            direction: String,
            v: u8,
        }

        let cmd = Command {
            command: "set_ort_teach",
            teach_type: teach_rotate_joint.to_string(),
            direction: direction.to_string(),
            v: if v > 100 { 100 } else { v },
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmOrtTeachResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    /// 示教停止
    fn set_stop_teach(&mut self) -> Result<ArmStopTeachResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
        }

        let cmd = Command {
            command: "set_stop_teach",
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<ArmStopTeachResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }
}
