use std::{thread::sleep, time::Duration};

use realman::{
    traits::{IOTrait, ModbusTrait, MotionTrait},
    ArmType, Realman, TrajectoryConnect,
};

#[test]
fn test_movej() {
    let realman = Realman::default();

    match realman.connect() {
        Ok(mut socket) => {
            socket
                .set_tool_voltage(realman::VoltageType::Three)
                .unwrap();
            socket
                .set_modbus_mode(
                    realman::PortType::EndRS485RTUMain,
                    realman::BaudrateType::Baud115200,
                    1,
                )
                .unwrap();

            socket
                .movej(
                    ArmType::Gen72([10100, 200, 10300, 30400, 500, 20600, 20600]),
                    100,
                    0,
                    TrajectoryConnect::Now,
                )
                .unwrap();
            socket
                .movej(
                    ArmType::Gen72([20100, 200, 20300, 40400, 500, 20600, 10600]),
                    100,
                    0,
                    TrajectoryConnect::Now,
                )
                .unwrap();
            socket
                .movej(
                    ArmType::Gen72([30100, 200, 30300, 20400, 500, 20600, 600]),
                    100,
                    0,
                    TrajectoryConnect::Now,
                )
                .unwrap();
            socket
                .movej(
                    ArmType::Gen72([-10100, 200, -20300, 30400, 500, 20600, -20600]),
                    100,
                    0,
                    TrajectoryConnect::Now,
                )
                .unwrap();

            socket
                .write_single_register(realman::ModbusPortType::EndRS485RTUMain, 40000, 100, 1)
                .unwrap();
            sleep(Duration::from_secs(1));
            socket
                .write_single_register(realman::ModbusPortType::EndRS485RTUMain, 40000, 0, 1)
                .unwrap();
            sleep(Duration::from_secs(1));

            socket
                .movej(
                    ArmType::Gen72([0, 0, 0, 0, 0, 0, 0]),
                    100,
                    0,
                    TrajectoryConnect::Now,
                )
                .unwrap();
        }
        Err(err) => panic!("{err:?}"),
    }
}
