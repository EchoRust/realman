use std::io::{Read, Write};

use serde::Serialize;

use crate::{
    traits::{IOData, IOTrait, IOVoltageQueryData},
    Result, Transport, VoltageType,
};

impl IOTrait for Transport {
    fn set_tool_voltage(&mut self, voltage_type: VoltageType) -> Result<IOData> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            voltage_type: u8,
        }

        let cmd = Command {
            command: "set_tool_voltage",
            voltage_type: voltage_type.into(),
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp = serde_json::from_str::<IOData>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    fn get_tool_voltage(&mut self) -> Result<IOVoltageQueryData> {
        todo!()
    }
}
