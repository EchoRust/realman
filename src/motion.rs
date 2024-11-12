//! 运动指令集

use std::io::{Read, Write};

use log::info;
use serde::Serialize;

use crate::{
    traits::{MotionData, MotionTrait},
    ArmType, Result, TrajectoryConnect, Transport,
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
}
