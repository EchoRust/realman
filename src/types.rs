use std::fmt::Display;

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 关节角度
///
/// 第一关节 -172度到172度。-172000 ～ 172000
/// 第二关节 -90度到90度。-90000 ～ 90000
#[derive(Debug, Clone, Copy)]
pub enum ArmType {
    Gen72([i32; 7]),
}

impl From<ArmType> for Vec<i32> {
    fn from(value: ArmType) -> Self {
        match value {
            ArmType::Gen72(data) => data.to_vec(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum TrajectoryConnect {
    Now = 0,
    Next = 1,
}

impl From<TrajectoryConnect> for u8 {
    fn from(value: TrajectoryConnect) -> Self {
        value as u8
    }
}

// 电源输出类型
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VoltageType {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}
impl From<VoltageType> for u8 {
    fn from(value: VoltageType) -> Self {
        value as u8
    }
}

// 通讯端口
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PortType {
    ControlRS485RTUMain = 0,
    EndRS485RTUMain = 1,
    ControlRS485RTUSlave = 2,
}
impl From<PortType> for u8 {
    fn from(value: PortType) -> Self {
        value as u8
    }
}

// ModBus寄存器通讯端口
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModbusPortType {
    ControlRS485RTUMain = 0,
    EndRS485RTUMain = 1,
    ControlModbusTCPConnectSlave = 3,
}
impl From<ModbusPortType> for u8 {
    fn from(value: ModbusPortType) -> Self {
        value as u8
    }
}

// 波特率
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaudrateType {
    /// 110 baud.
    Baud110,

    /// 300 baud.
    Baud300,

    /// 600 baud.
    Baud600,

    /// 1200 baud.
    Baud1200,

    /// 2400 baud.
    Baud2400,

    /// 4800 baud.
    Baud4800,

    /// 9600 baud.
    Baud9600,

    /// 19,200 baud.
    Baud19200,

    /// 38,400 baud.
    Baud38400,

    /// 57,600 baud.
    Baud57600,

    /// 115,200 baud.
    Baud115200,

    /// Non-standard baud rates.
    ///
    /// `BaudOther` can be used to set non-standard baud rates by setting its member to be the
    /// desired baud rate.
    ///
    /// Non-standard baud rates may not be supported on all systems.
    BaudOther(usize),
}
impl BaudrateType {
    pub fn from_speed(speed: usize) -> BaudrateType {
        match speed {
            110 => BaudrateType::Baud110,
            300 => BaudrateType::Baud300,
            600 => BaudrateType::Baud600,
            1200 => BaudrateType::Baud1200,
            2400 => BaudrateType::Baud2400,
            4800 => BaudrateType::Baud4800,
            9600 => BaudrateType::Baud9600,
            19200 => BaudrateType::Baud19200,
            38400 => BaudrateType::Baud38400,
            57600 => BaudrateType::Baud57600,
            115200 => BaudrateType::Baud115200,
            n => BaudrateType::BaudOther(n),
        }
    }

    pub fn speed(&self) -> usize {
        match *self {
            BaudrateType::Baud110 => 110,
            BaudrateType::Baud300 => 300,
            BaudrateType::Baud600 => 600,
            BaudrateType::Baud1200 => 1200,
            BaudrateType::Baud2400 => 2400,
            BaudrateType::Baud4800 => 4800,
            BaudrateType::Baud9600 => 9600,
            BaudrateType::Baud19200 => 19200,
            BaudrateType::Baud38400 => 38400,
            BaudrateType::Baud57600 => 57600,
            BaudrateType::Baud115200 => 115200,
            BaudrateType::BaudOther(n) => n,
        }
    }
}

// 位置步进类型
pub enum StepType {
    XStep,
    YStep,
    ZStep,
}

impl Display for StepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::XStep => "x_step",
                Self::YStep => "y_step",
                Self::ZStep => "z_step",
            }
        )
    }
}

// 姿态步进类型
pub enum RStepType {
    RXStep,
    RYStep,
    RZStep,
}

impl Display for RStepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::RXStep => "rx_step",
                Self::RYStep => "ry_step",
                Self::RZStep => "rz_step",
            }
        )
    }
}
