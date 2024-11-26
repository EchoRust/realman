use std::io::{Read, Write};

use serde::Serialize;

use crate::{
    traits::{ArmStateData, ArmStateTrait, JointDegreeData},
    Result, Transport,
};

impl ArmStateTrait for Transport {
    fn get_current_arm_state(&mut self) -> Result<ArmStateData> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
        }

        let cmd = Command {
            command: "get_current_arm_state",
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp = serde_json::from_str::<ArmStateData>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    fn get_joint_degree(&mut self) -> Result<JointDegreeData> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
        }

        let cmd = Command {
            command: "get_joint_degree",
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<JointDegreeData>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }
}
