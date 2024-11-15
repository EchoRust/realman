// 乐白LMG-90夹抓示例
//
// * 1、先要进行末端电压设置，为24V
// * 2、配置末端通讯接口为ModbusRTU模式
// * 3、LMG-90夹抓的开关，注意，modbus命令发出后会立即返回结果，夹抓开始工作到工作结束需要1秒钟的时间，需要等待1秒钟后才可以进行关闭夹抓的操作，否则夹抓会立即响应关闭的指令，表现为没有张开夹抓的动作。
//

use std::{thread::sleep, time::Duration};

use realman::{
    traits::{IOTrait, ModbusTrait, MotionTrait},
    ArmType, Realman, TrajectoryConnect,
};

fn main() {
    let realman = Realman::default();

    if let Ok(mut socket) = realman.connect() {
        // 设置工具端电源输出
        socket.set_tool_voltage(realman::VoltageType::Three).ok();

        // 配置通讯端口ModbusRTU模式
        socket
            .set_modbus_mode(
                realman::PortType::EndRS485RTUMain,
                realman::BaudrateType::Baud115200,
                1,
            )
            .ok();

        // 回到零位
        socket
            .movej(ArmType::Gen72([0; 7]), 50, 0, TrajectoryConnect::Now)
            .ok();

        // 张开夹抓
        socket
            .write_single_register(realman::ModbusPortType::EndRS485RTUMain, 40000, 100, 1)
            .ok();
        // 等1秒钟
        sleep(Duration::from_secs(1));
        // 关闭夹抓
        socket
            .write_single_register(realman::ModbusPortType::EndRS485RTUMain, 40000, 0, 1)
            .ok();
        // 等1秒钟
        sleep(Duration::from_secs(1));
    }
}
