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
    /// * voltage_type 电源输出类型，范围：0~3。0-0V，2-12V，3-24V
    fn set_tool_voltage(&mut self, voltage_type: VoltageType) -> Result<IOData>;

    /// 查询工具端电源输出
    ///
    /// * voltage_type 电源输出类型，范围：0~3。0-0V，2-12V，3-24V
    fn get_tool_voltage(&mut self) -> Result<IOVoltageQueryData>;
}
