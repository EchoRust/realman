// 上下左右示例
//
// 机械臂先进入到开始为之后，开始前后左右上下，旋转等动作
// 最后回到零位置
//

use realman::{traits::MotionTrait, ArmType, Realman, TrajectoryConnect};

fn main() {
    let realman = Realman::default();

    if let Ok(mut socket) = realman.connect() {
        // 进入到开始位置
        socket
            .movej(
                ArmType::Gen72([0, -60000, 0, -60000, 0, 0, 0]),
                50,
                0,
                TrajectoryConnect::Now,
            )
            .ok();
        // 前后
        socket
            .set_pos_step(realman::StepType::XStep, 80000, 30)
            .ok();
        socket
            .set_pos_step(realman::StepType::XStep, -80000, 30)
            .ok();
        // 左右
        socket
            .set_pos_step(realman::StepType::YStep, 50000, 30)
            .ok();
        socket
            .set_pos_step(realman::StepType::YStep, -50000, 30)
            .ok();
        // 上下
        socket
            .set_pos_step(realman::StepType::ZStep, 30000, 30)
            .ok();
        socket
            .set_pos_step(realman::StepType::ZStep, -30000, 30)
            .ok();

        // X轴旋转
        socket
            .set_ort_step(realman::RStepType::RXStep, -500, 30)
            .ok();
        socket
            .set_ort_step(realman::RStepType::RXStep, 500, 30)
            .ok();
        // Y轴旋转
        socket
            .set_ort_step(realman::RStepType::RYStep, -500, 30)
            .ok();
        socket
            .set_ort_step(realman::RStepType::RYStep, 500, 30)
            .ok();
        // Z轴旋转
        socket
            .set_ort_step(realman::RStepType::RZStep, -500, 30)
            .ok();
        socket
            .set_ort_step(realman::RStepType::RZStep, 500, 30)
            .ok();

        // 回到零位
        socket
            .movej(ArmType::Gen72([0; 7]), 50, 0, TrajectoryConnect::Now)
            .ok();
    }
}
