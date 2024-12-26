use std::fmt::Display;

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 机械臂类型
// 第一关节 -172度到172度。-172000 ～ 172000
// 第二关节 -90度到90度。-90000 ～ 90000
#[derive(Debug, Clone, Copy)]
pub enum ArmType {
    /// RM65机器人本体模仿人的手臂，共有6个旋转关节，每个关节表示1个自由度。机器人关节包括肩部（关节1），肩部（关节2），肘部（关节3），腕部（关节4），腕部（关节5）和腕部（关节6）。
    ///
    /// ![RM65](https://develop.realman-robotics.com/assets/RM_65_image1.1fMZoZdk.png)
    Rm65([i32; 6]),
    /// RM75机器人本体模仿人的手臂，共有7个旋转关节，每个关节表示1个自由度。机器人关节包括肩部（关节1），肩部（关节2），肩部（关节3），肘部（关节4），腕部（关节5），腕部（关节6）和腕部（关节7）。
    ///
    /// ![RM75](https://develop.realman-robotics.com/assets/RM_75_image1.B3YxHg-k.png)
    Rm75([i32; 7]),
    /// RML63机器人本体模仿人的手臂，共有6个旋转关节，每个关节表示1个自由度。机器人关节包括肩部（关节1），肩部（关节2），肘部（关节3），腕部（关节4），腕部（关节5）和腕部（关节6）。
    ///
    /// ![RML63](https://develop.realman-robotics.com/assets/RML_63_image1.DlaXwnm6.png)
    Rml63([i32; 6]),
    /// ECO65机器人本体模仿人的手臂，共有6个旋转关节，每个关节表示1个自由度。机器人关节包括肩部（关节1），肩部（关节2），肘部（关节3），腕部（关节4），腕部（关节5）和腕部（关节6）。
    ///
    /// ![ECO65](https://develop.realman-robotics.com/assets/ECO_65_image1.BKLb4BaS.png)
    Eco65([i32; 6]),
    /// GEN72机器人共有 7 个旋转关节，每个关节表示 1 个自由度。机器人关节包括肩部（关节 1），肩部（关节 2），肩部（关节 3），肘部（关节 4），腕部（关节5），腕部（关节 6）和腕部（关节 7）。
    ///
    /// ![Gen72](https://develop.realman-robotics.com/assets/GEN_72_image1.V_sv9b12.png)
    Gen72([i32; 7]),
}

impl From<ArmType> for Vec<i32> {
    fn from(value: ArmType) -> Self {
        match value {
            ArmType::Gen72(data) => data.to_vec(),
            ArmType::Rm65(data) => data.to_vec(),
            ArmType::Rm75(data) => data.to_vec(),
            ArmType::Rml63(data) => data.to_vec(),
            ArmType::Eco65(data) => data.to_vec(),
        }
    }
}

/// 是否和下一条运动一起规划类型
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum TrajectoryConnect {
    /// 代表立即规划
    Now = 0,
    /// 和下一条轨迹一起规划，当为此选项时，轨迹不会立即执行
    Next = 1,
}

impl From<TrajectoryConnect> for u8 {
    fn from(value: TrajectoryConnect) -> Self {
        value as u8
    }
}

/// 电源输出类型
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VoltageType {
    /// 0V
    Zero = 0,
    /// 5V
    One = 1,
    /// 12V
    Two = 2,
    /// 24V
    Three = 3,
}

impl From<VoltageType> for u8 {
    fn from(value: VoltageType) -> Self {
        value as u8
    }
}

/// 通讯端口
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PortType {
    /// 控制器RS485端口为RTU主站
    ControlRS485RTUMain = 0,
    /// 末端接口板RS485接口为RTU主站
    EndRS485RTUMain = 1,
    /// 控制器RS485端口为RTU从站
    ControlRS485RTUSlave = 2,
}

impl From<PortType> for u8 {
    fn from(value: PortType) -> Self {
        value as u8
    }
}

/// ModBus寄存器通讯端口
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModbusPortType {
    /// 控制器RS485端口
    ControlRS485RTUMain = 0,
    /// 末端接口板RS485接口
    EndRS485RTUMain = 1,
    /// 控制器ModbusTCP连接外部从站端口
    ControlModbusTCPConnectSlave = 3,
}

impl From<ModbusPortType> for u8 {
    fn from(value: ModbusPortType) -> Self {
        value as u8
    }
}

/// 波特率
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(usize)]
pub enum BaudrateType {
    /// 9600波特率.
    Baud9600 = 9600,
    /// 115200波特率.
    Baud115200 = 115200,
    /// 460800波特率.
    Baud460800 = 460800,
}

impl From<BaudrateType> for usize {
    fn from(value: BaudrateType) -> Self {
        value as usize
    }
}

/// 位置步进类型
pub enum StepType {
    /// X 轴方向
    XStep,
    /// Y 轴方向
    YStep,
    /// Z 轴方向
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

/// 姿态步进类型
pub enum RStepType {
    /// 绕 X 轴旋转
    RXStep,
    /// 绕 Y 旋转
    RYStep,
    /// 绕 Z 轴旋转
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

/// 示教方向
pub enum DirectionType {
    /// 正方向
    Pos,
    /// 反方向
    Neg,
}

impl Display for DirectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pos => "pos",
                Self::Neg => "neg",
            }
        )
    }
}

/// 示教坐标轴
pub enum TeachType {
    /// X轴
    X,
    /// Y轴
    Y,
    /// Z轴
    Z,
}

impl Display for TeachType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "x",
                Self::Y => "y",
                Self::Z => "z",
            }
        )
    }
}

/// 示教旋转所绕坐标轴
pub enum TeachRotateType {
    /// rx轴
    Rx,
    /// ry轴
    Ry,
    /// rz轴
    Rz,
}

impl Display for TeachRotateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Rx => "rx",
                Self::Ry => "ry",
                Self::Rz => "rz",
            }
        )
    }
}
