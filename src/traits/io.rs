//! IO配置指令集 `trait`
//!
//! ## 末端工具IO
//! 机械臂末端工具端具有IO端口，数量和分类如下所示：
//! |类型|说明|
//! |---|---|
//! |电源输出|1路，可配置为0V/5V/12V/24V。|
//! |数字IO|2路，输入输出可配置。|
//! |通讯接口|1路，可配置为RS485。|
//!
//! * 设置工具端电源输出 `set_tool_voltage`
//! * 查询工具端电源输出 `get_tool_voltage`

use serde::{Deserialize, Serialize};

use crate::{Result, VoltageType};

#[derive(Debug, Serialize, Deserialize)]
pub struct IOData {
    command: String,
    state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IOVoltageQueryData {
    state: String,
    voltage_type: VoltageType,
}

pub trait IOTrait {
    /// 设置工具端电源输出
    ///
    /// * voltage_type 电源输出类型，范围：0~3。0-0V，1-5V，2-12V，3-24V
    fn set_tool_voltage(&mut self, voltage_type: VoltageType) -> Result<IOData>;

    /// 查询工具端电源输出
    ///
    /// * voltage_type 电源输出类型，范围：0~3。0-0V，1-5V，2-12V，3-24V
    fn get_tool_voltage(&mut self) -> Result<IOVoltageQueryData>;
}
