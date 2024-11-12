use std::{
    io::{Read, Write},
    thread::sleep,
    time::Duration,
};

use serde::Serialize;

use crate::{
    traits::{ModbusTrait, SetStateResponse, WriteStateResponse},
    BaudrateType, ModbusPortType, PortType, Result, Transport,
};

impl ModbusTrait for Transport {
    fn set_modbus_mode(
        &mut self,
        port: PortType,
        baudrate: BaudrateType,
        timeout: u8,
    ) -> Result<SetStateResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            port: u8,
            baudrate: usize,
            timeout: u8,
        }

        let cmd = Command {
            command: "set_modbus_mode",
            port: port.into(),
            baudrate: baudrate.speed(),
            timeout: if timeout == 0 { 1 } else { timeout },
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        sleep(Duration::from_millis(10));

        let cmd_resp =
            serde_json::from_str::<SetStateResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }

    fn write_single_register(
        &mut self,
        port: ModbusPortType,
        address: u16,
        data: i16,
        device: i32,
    ) -> Result<WriteStateResponse> {
        let mut buf = [0; 256];

        #[derive(Serialize)]
        struct Command {
            command: &'static str,
            port: u8,
            address: u16,
            data: i16,
            device: i32,
        }

        let cmd = Command {
            command: "write_single_register",
            port: port.into(),
            address,
            data,
            device,
        };

        let data = serde_json::to_vec(&cmd)?;

        self.write_all(&data)?;

        let n = self.read(&mut buf)?;

        let cmd_resp =
            serde_json::from_str::<WriteStateResponse>(&String::from_utf8_lossy(&buf[0..n]))?;

        Ok(cmd_resp)
    }
}
