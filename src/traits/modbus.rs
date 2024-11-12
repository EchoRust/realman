use crate::{BaudrateType, ModbusPortType, PortType, Result};

use super::{SetStateResponse, WriteStateResponse};

pub trait ModbusTrait {
    fn set_modbus_mode(
        &mut self,
        port: PortType,
        baudrate: BaudrateType,
        timeout: u8,
    ) -> Result<SetStateResponse>;

    /// 写多个寄存器
    ///
    /// * port 通讯端口，0-控制器RS485端口，1-末端接口板RS485接口，3-控制器ModbusTCP连接外部从站端口。
    /// * address 线圈起始地址。
    /// * data 要写入线圈的数据。
    /// * device 外设设备地址。
    fn write_single_register(
        &mut self,
        port: ModbusPortType,
        address: u16,
        data: i16,
        device: i32,
    ) -> Result<WriteStateResponse>;
}
