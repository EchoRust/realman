// 点头示例
//
// 机械臂连接上后进行到初始化零位置后，进行3次点头操作
//

use realman::{traits::MotionTrait, ArmType, Realman, TrajectoryConnect};

fn main() {
    let realman = Realman::default();

    if let Ok(mut socket) = realman.connect() {
        socket
            .movej(ArmType::Gen72([0; 7]), 50, 0, TrajectoryConnect::Now)
            .ok();

        for _ in 0..3 {
            socket.set_joint_step(&[6, -15000], 50).ok();
            socket.set_joint_step(&[6, 15000], 50).ok();
        }
    }
}
