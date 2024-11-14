use std::{thread::sleep, time::Duration};

use realman::{
    traits::{ArmStateTrait, IOTrait, ModbusTrait, MotionTrait},
    ArmType, Realman, TrajectoryConnect,
};

fn main() {
    let realman = Realman::default();

    match realman.connect() {
        Ok(mut socket) => {
            socket.set_tool_voltage(realman::VoltageType::Three).ok();
            socket
                .set_modbus_mode(
                    realman::PortType::EndRS485RTUMain,
                    realman::BaudrateType::Baud115200,
                    1,
                )
                .ok();

            let res = socket.get_current_arm_state().unwrap();
            println!("{res:#?}");

            socket
                .movej(
                    ArmType::Gen72([0, -60000, 0, -60000, 0, 0, 0]),
                    50,
                    0,
                    TrajectoryConnect::Now,
                )
                .ok();

            socket
                .set_pos_step(realman::StepType::XStep, 80000, 30)
                .ok();
            socket
                .set_pos_step(realman::StepType::XStep, -80000, 30)
                .ok();
            socket
                .set_pos_step(realman::StepType::YStep, 50000, 30)
                .ok();
            socket
                .set_pos_step(realman::StepType::YStep, -50000, 30)
                .ok();
            socket
                .set_pos_step(realman::StepType::ZStep, 30000, 30)
                .ok();
            socket
                .set_pos_step(realman::StepType::ZStep, -30000, 30)
                .ok();

            socket
                .set_ort_step(realman::RStepType::RXStep, -500, 30)
                .ok();
            socket
                .set_ort_step(realman::RStepType::RXStep, 500, 30)
                .ok();
            socket
                .set_ort_step(realman::RStepType::RYStep, -500, 30)
                .ok();
            socket
                .set_ort_step(realman::RStepType::RYStep, 500, 30)
                .ok();
            socket
                .set_ort_step(realman::RStepType::RZStep, -500, 30)
                .ok();
            socket
                .set_ort_step(realman::RStepType::RZStep, 500, 30)
                .ok();

            socket.set_joint_step(&[6, -10000], 30).ok();
            socket.set_joint_step(&[6, 10000], 30).ok();

            socket
                .write_single_register(realman::ModbusPortType::EndRS485RTUMain, 40000, 100, 1)
                .ok();
            sleep(Duration::from_secs(1));
            socket
                .write_single_register(realman::ModbusPortType::EndRS485RTUMain, 40000, 0, 1)
                .ok();
            sleep(Duration::from_secs(1));

            socket
                .movej(
                    ArmType::Gen72([0, 0, 0, 0, 0, 0, 0]),
                    50,
                    0,
                    TrajectoryConnect::Now,
                )
                .ok();
        }
        Err(err) => panic!("{err:?}"),
    }
}
