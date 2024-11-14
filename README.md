# 睿尔曼机械臂 Rust SDK

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/badge/crates-0.0.2-yellow
[crates-url]: https://crates.io/crates/realman
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT

这是睿尔曼机械臂的 `Rust SDK` ，旨在帮助 `Rust` 开发者更好的对睿尔曼机械臂进行二次开发。

`SDK` 是根据官网 `json` 协议开发。[官网链接](https://develop.realman-robotics.com/robot/json/getStartedJson.html)

## 作者介绍

* 作者: 李扬(Leon)
* 个人网站: 技安Rust笔记 - [https://echoli.cn](https://echoli.cn)
* Github: [https://github.com/EchoRust/realman](https://github.com/EchoRust/realman)

## 使用方法

<details>
<summary>
点击显示 `Cargo.toml`
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8f947f954da465399326296f113be8a7" target="_blank">在playground中运行当前代码</a>
</summary>

```toml
[dependencies]
realman = "0.0.2"
```
</details>

```rust
use realman::{
    traits::{ArmStateTrait, IOTrait, ModbusTrait, MotionTrait},
    ArmType, Realman, TrajectoryConnect,
};

fn main() {
    // get default realman
    let realman = Realman::default();

    match realman.connect() {
        Ok(mut socket) => {
            // set end voltage
            socket.set_tool_voltage(realman::VoltageType::Three).ok();

            // set modbus mode
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
        }
        Err(err) => panic!("{err:?}"),
    }
}
```

## License

<sup>
The MIT License (MIT)

Copyright (c) 2024-present, Leon 李扬

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</sup>
