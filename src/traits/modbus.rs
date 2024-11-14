//! `Modbus` 指令集 `trait`
//!
//! ## Modbus 模式配置
//!
//! 睿尔曼机械臂在控制器的航插和末端接口板航插处，各有1路RS485通讯接口，
//! 这两个RS485端口可通过JSON协议配置为标准的ModbusRTU模式。
//! 然后通过JSON协议对端口连接的外设进行读写操作。
//!
//! 注意：控制器的RS485接口在未配置为ModbusRTU模式的情况下，
//! 可用于用户对机械臂进行控制，这两种模式不可兼容。
//! 若要恢复机械臂控制模式，必须将该端口的ModbusRTU模式关闭。
//! ModbusRTU模式关闭后，系统会自动切换回机械臂控制模式，
//! 波特率460800BPS，停止位1，数据位8，无检验。
//!
//! 同时，第三代控制器支持ModbusTCP主站配置，可配置使用ModbusTCP主站，
//! 用于连接外部设备的ModbusTCP从站。
//!

use crate::{BaudrateType, ModbusPortType, PortType, Result};

use super::{SetStateResponse, WriteStateResponse};

pub trait ModbusTrait {
    /// 配置通讯端口ModbusRTU模式
    ///
    /// 配置通讯端口ModbusRTU模式，机械臂启动后，要对通讯端口进行任何操作，
    /// 必须先启动该指令，否则会返回报错信息。
    ///
    /// * `port` 通讯端口，0-控制器RS485端口为RTU主站，1-末端接口板RS485接口为RTU主站，2-控制器RS485端口为RTU从站。
    /// * `baudrate` 波特率，支持9600,115200,460800三种常见波特率。
    /// * `timeout` 超时时间，单位百毫秒。对Modbus设备所有的读写指令，在规定的超时时间内未返回响应数据，则返回超时报错提醒。超时时间不能为0，若设置为0，则机械臂按1进行配置。
    fn set_modbus_mode(
        &mut self,
        port: PortType,
        baudrate: BaudrateType,
        timeout: u8,
    ) -> Result<SetStateResponse>;

    /// 写单个寄存器
    ///
    /// * `port` 通讯端口，0-控制器RS485端口，1-末端接口板RS485接口，3-控制器ModbusTCP连接外部从站端口。
    /// * `address` 线圈起始地址。
    /// * `data` 要写入线圈的数据。
    /// * `device` 外设设备地址。
    fn write_single_register(
        &mut self,
        port: ModbusPortType,
        address: u16,
        data: i16,
        device: i32,
    ) -> Result<WriteStateResponse>;
}
