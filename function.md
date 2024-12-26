## 功能列表

* [ ] 关节参数指令集 `jointParameter`
  * [ ] 设置关机配置
    * [ ] 设置关节最大转速 `set_joint_max_speed`
    * [ ] 设置关节最大加速度 `set_joint_max_acc`
    * [ ] 设置关节最小限位 `set_joint_min_pos`
    * [ ] 设置关节最大限位 `set_joint_max_pos`
    * [ ] 设置关节最大转速（驱动器） `set_joint_drive_max_speed`
    * [ ] 设置关节最大加速度（驱动器） `set_joint_drive_max_acc`
    * [ ] 设置关节最小限位（驱动器） `set_joint_drive_min_pos`
    * [ ] 设置关节最大限位（驱动器） `set_joint_drive_max_pos`
    * [ ] 设置关节使能状态 `set_joint_en_state`
    * [ ] 设置关节零位 `set_joint_zero_pos`
    * [ ] 清除关节错误代码 `set_joint_clear_err`
    * [ ] 一键设置关节限位 `auto_set_joint_limit`
  * [ ] 查询关节配置
    * [ ] 查询关节最大速度 `get_joint_max_speed`
    * [ ] 查询关节最大加速度 `get_joint_max_acc`
    * [ ] 查询关节最小限位 `get_joint_min_pos`
    * [ ] 查询关节最大限位 `get_joint_max_pos`
    * [ ] 查询关节最大速度（驱动器） `get_joint_min_pos`
    * [ ] 查询关节最大加速度（驱动器） `get_joint_drive_max_acc`
    * [ ] 查询关节最小限位（驱动器） `get_joint_drive_min_pos`
    * [ ] 查询关节最大限位（驱动器） `get_joint_drive_max_pos`
    * [ ] 查询关节使能状态 `get_joint_en_state`
    * [ ] 查询关节错误代码 `get_joint_err_flag`
* [ ] 运动参数指令集 `armConfig`
  * [ ] 设置运动参数
    * [ ] 设置末端最大线速度 `set_arm_max_line_speed`
    * [ ] 设置末端最大线加速度 `set_arm_max_line_acc`
    * [ ] 设置末端最大角速度 `set_arm_max_angular_speed`
    * [ ] 设置末端最大角加速度 `set_arm_max_angular_acc`
    * [ ] 初始化机械臂参数 `set_arm_init`
    * [ ] 设置碰撞防护等级 `set_collision_stage`
    * [ ] 查询碰撞防护等级 `get_collision_stage`
    * [ ] 设置自碰撞安全检测使能状态 `set_self_collision_enable`
    * [ ] 获取自碰撞安全检测使能状态 `get_self_collision_enable`
    * [ ] 重新设置 DH 参数（第三代控制器） `set_DH_data`
    * [ ] 查询 DH 参数（第三代控制器） `get_DH_data`
    * [ ] 恢复机械臂默认 DH 参数（第三代控制器） `set_DH_data_default`
    * [ ] 重设关节零位补偿 `set_joint_zero_offset`
  * [ ] 查询运动参数
    * [ ] 查询末端最大线速度 `get_arm_max_line_speed`
    * [ ] 查询末端最大线加速度 `get_arm_max_line_acc`
    * [ ] 查询末端最大角速度 `get_arm_max_angular_speed`
    * [ ] 查询末端最大角加速度 `get_arm_max_angular_acc`
* [ ] 运动指令集 `motionConfig`
  * [ ] 轨迹运动
    * [x] 关节运动 `movej`
    * [ ] 直线运动 `movel`
    * [ ] 圆弧运动 `movec`
    * [ ] 关节空间规划到目标位姿 `movej_p`
    * [ ] 样条曲线运动 `moves`
    * [ ] 位姿透传 `movep_canfd`
    * [ ] 角度透传 `movej_canfd`
    * [ ] 关节空间跟随运动 `movej_follow`
    * [ ] 笛卡尔空间跟随运动 `movep_follow`
  * [ ] 力位混合运动（选配）
    * [ ] 力位混合控制 `set_force_position`
    * [ ] 结束力位混合控制 `stop_force_position`
    * [ ] 透传力位混合控制补偿（选配） `Start_Force_Position_Move`
      * [ ] 开启透传力位混合控制补偿模式
      * [ ] 透传力位混合补偿
      * [ ] 关闭透传力位混合控制补偿模式
  * [ ] 运动控制
    * [x] 轨迹急停 `set_arm_stop`
    * [x] 轨迹缓停 `set_arm_slow_stop`
    * [x] 轨迹暂停 `set_arm_pause`
    * [x] 轨迹暂停后恢复 `set_arm_continue`
    * [ ] 清除当前轨迹 `set_delete_current_trajectory`
    * [ ] 清除所有轨迹 `set_arm_delete_trajectory`
    * [ ] 查询当前规划类型 `get_arm_current_trajectory`
    * [ ] 轨迹结束返回标志 `current_trajectory_state`
  * [x] 步进运动
    * [x] 关节步进 `set_joint_step`
    * [x] 位置步进 `set_pos_step`
    * [x] 姿态步进 `set_ort_step`
  * [x] 示教运动
    * [x] 关节示教 `set_joint_teach`
    * [x] 位置示教 `set_pos_teach`
    * [x] 姿态示教 `set_ort_teach`
    * [x] 示教停止 `set_stop_teach`
    * [ ] 设置示教参考坐标系 `set_teach_frame`
    * [ ] 获取示教参考坐标系 `get_teach_frame`
* [ ] 坐标系指令集 `coordinate`
  * [ ] 工具坐标系
    * [ ] 自动计算工具坐标系（标定参考点） `set_auto_tool_frame`
    * [ ] 自动计算工具坐标系（自动计算生成工具） `set_auto_tool_frame`
    * [ ] 手动输入工具坐标系 `set_manual_tool_frame`
    * [ ] 切换当前工具坐标系 `set_change_tool_frame`
    * [ ] 删除工具坐标系 `set_delete_tool_frame`
    * [ ] 修改工具坐标系 `update_tool_frame`
    * [ ] 设置工具坐标系包络参数 `set_tool_envelope`
    * [ ] 查询工具坐标系包络参数 `get_tool_envelope`
    * [ ] 查询当前工具坐标系 `get_current_tool_frame`
    * [ ] 查询已有所有工具名称 `get_total_tool_frame`
    * [ ] 查询指定工具坐标系 `get_tool_frame`
  * [ ] 工作坐标系
    * [ ] 自动设置工作坐标系 `set_auto_work_frame`
    * [ ] 手动输入工作坐标系 `set_manual_work_frame`
    * [ ] 切换当前工作坐标系 `set_change_work_frame`
    * [ ] 删除工作坐标系 `set_delete_work_frame`
    * [ ] 修改工作坐标系 `update_work_frame`
    * [ ] 查询当前工作坐标系 `get_current_work_frame`
    * [ ] 查询已有所有工作坐标系名称 `get_total_work_frame`
    * [ ] 查询指定工作坐标系 `get_work_frame`
* [ ] 状态查询指令集 `armState`
  * [ ] 机械臂状态
    * [x] 查询机械臂状态 `get_current_arm_state`
    * [ ] 查询控制器状态 `get_controller_state`
    * [ ] 清除系统错误 `clear_system_err`
    * [ ] 查询关节温度 `get_current_joint_temperature`
    * [ ] 查询关节当前电流 `get_current_joint_current`
    * [ ] 查询关节当前电压 `get_current_joi`nt_voltage`
    * [x] 查询机械臂关节角度 `get_joint_degree`
    * [ ] 查询机械臂所有状态信息 `get_arm_all_state`
    * [ ] 查询规划计数 `get_arm_plan_num`
  * [ ] 初始姿态
    * [ ] 设置初始状态 `set_init_pose`
    * [ ] 查询初始位置 `get_init_pose`
  * [ ] 安装方式
    * [ ] 设置安装方式参数 `set_install_pose`
    * [ ] 查询安装方式参数 `get_install_pose`
* [ ] 系统指令集 `systemConfig`
  * [ ] 电源控制
    * [ ] 控制上电与断电 `set_arm_power`
    * [ ] 读取电源状态 `get_arm_power_state`
  * [ ] 版本信息
    * [ ] 查询软件版本（第二代控制器） `get_arm_software_version`
    * [ ] 查询软件版本（第三代控制器） `get_arm_software_info`
    * [ ] 查询关节软件版本号 `get_joint_software_version`
    * [ ] 查询末端接口板软件版本号 `get_tool_software_version`
  * [ ] 累计数据
    * [ ] 读取控制器的累计运行时间 `get_system_runtime`
    * [ ] 清零控制器的累计运行时间 `clear_system_runtime`
    * [ ] 读取关节的累计转动角度 `get_joint_odom`
    * [ ] 清零关节的累计转动角度 `clear_joint_odom`
  * [ ] 参数保存
    * [ ] 参数保存（第二代） `save_device_info_all`
* [ ] 网络配置指令集 `networkConfig`
  * [ ] 有线网络
    * [ ] 高速网口使能配置（第二代控制器） `set_high_speed_eth`
    * [ ] 设置高速网口IP（第二代控制器） `set_high_ethernet`
    * [ ] 查询高速网口IP（第二代控制器） `get_high_ethernet`
    * [ ] 配置有线网络信息（第三代控制器） `set_NetIP`
    * [ ] 查询有线网卡网络信息（第三代控制器） `get_wired_net`
  * [ ] 无线网络
    * [ ] 查询无线网卡网络信息（第三代控制器） `get_wifi_net`
    * [ ] 关闭wifi功能（第三代控制器） `set_wifi_close`
    * [ ] 设置wifiAP  `set_wifi_ap`
    * [ ] 设置wifiSTA  `set_wifi_sta`
  * [ ] 恢复网络
    * [ ] 恢复网络设置（第三代） `set_net_default`
  * [ ] 其他
    * [ ] 设置USB（第二代） `set_usb`
* [ ] IO配置指令集 `ioConfig`
  * [ ] 控制器IO
    * [ ] 设置数字IO模式（第三代控制器） `set_IO_mode`
    * [ ] 设置数字IO输出状态 `set_DO_state`
    * [ ] 查询数字IO状态（第三代控制器） `get_IO_state`
    * [ ] 查询数字IO输出状态（第二代控制器） `get_DO_state`
    * [ ] 查询数字IO输入状态（第二代控制器） `get_DI_state`
    * [ ] 设置模拟IO输出状态（第二代控制器） `set_AO_state`
    * [ ] 查询模拟IO输出状态（第二代控制器） `get_AO_state`
    * [ ] 查询模拟IO输入状态（第二代控制器） `get_AI_state`
    * [ ] 查询所有IO输入状态 `get_IO_input`
    * [ ] 查询所有IO输出状态 `get_IO_output`
    * [ ] 设置电源输出（第三代控制器） `set_voltage`
    * [ ] 查询电源输出（第三代控制器） `get_voltage`
  * [ ] 末端工具IO
    * [ ] 设置工具端数字IO输出状态 `set_tool_DO_state`
    * [ ] 设置工具端数字IO模式 `set_tool_IO_mode`
    * [ ] 查询工具端数字IO状态 `get_tool_IO_state`
    * [x] 设置工具端电源输出 `set_tool_voltage`
    * [x] 查询工具端电源输出 `get_tool_voltage`
* [ ] UDP主动上报指令集 `udpConfig`
  * [ ] UDP 主动上报
    * [ ] UDP 机械臂状态主动上报接口
    * [ ] 查询 UDP 机械臂状态主动上报配置 `get_realtime_push`
    * [ ] 设置 UDP 机械臂状态主动上报配置 `set_realtime_push`
* [ ] 在线编程指令集 `onlineProgram`
  * [ ] 文件传输
    * [ ] 下发前准备（第二代控制器） `run_project`
    * [ ] 下发前准备（第三代控制器） `run_project`
    * [ ] 发送过程中 `conduct_project`
    * [ ] 发送校验返回 `download_project`
    * [ ] 规划过程中改变速度系数 `plan_speed`
  * [ ] 提示框
    * [ ] 弹窗提示 `content`
  * [ ] 在线编程文件管理（第三代控制器）
    * [ ] 查询在线编程程序列表 `get_program_trajectory_list`
    * [ ] 查询在线编程状态 `get_program_run_state`
    * [ ] 开始运行指定编号在线编程文件 `set_program_id_start`
    * [ ] 删除指定编号在线编程文件 `delete_program_trajectory`
    * [ ] 修改指定编号的在线编程文件
    * [ ] 设置IO默认运行编号
    * [ ] 获取 IO 默认运行编号
  * [ ] 全局路点
    * [ ] 新增全局路点 `add_global_waypoint`
    * [ ] 更新全局路点 `update_global_waypoint`
    * [ ] 删除全局路点 `delete_global_waypoint`
    * [ ] 查询指定全局路点 `given_global_waypoint`
    * [ ] 查询多个全局路点 `get_global_waypoints_list`
* [ ] 拖动示教指令集 `dragTech`
  * [ ] 拖动示教
    * [ ] 拖动示教开始 `start_drag_teach`
    * [ ] 拖动示教结束 `stop_drag_teach`
    * [ ] 开始复合模式拖动示教 `start_multi_drag_teach`
    * [ ] 轨迹复现开始 `run_drag_trajectory`
    * [ ] 轨迹复现暂停 `pause_drag_trajectory`
    * [ ] 轨迹复现继续 `continue_drag_trajectory`
    * [ ] 轨迹复现停止 `stop_drag_trajectory`
    * [ ] 运动到轨迹起点 `drag_trajectory_origin`
    * [ ] 获取拖动示教轨迹 `save_trajectory`
    * [ ] 设置电流环拖动示教灵敏度 `set_drag_teach_sensitivity`
    * [ ] 获取电流环拖动示教灵敏度 `get_drag_teach_sensitivity`
* [ ] RS485与Modbus指令集 `modbus`
  * [ ] Modbus 模式配置
    * [ ] 设置RS485 `set_RS485`
    * [ ] 查询控制器RS485模式（第三代） `get_controller_RS485_mode`
    * [ ] 查询工具端RS485模式（第三代） `get_tool_RS485_mode`
    * [x] 配置通讯端口ModbusRTU模式 `set_modbus_mode`
    * [ ] 关闭通讯端口ModbusRTU模式 `close_modbus_mode`
    * [ ] 配置连接ModbusTCP从站（第三代控制器） `set_modbustcp_mode`
    * [ ] 置关闭ModbusTCP从站（第三代控制器） `close_modbustcp_mode`
  * [ ] Modbus 协议指令
    * [ ] 读线圈 `read_coils`
    * [ ] 读多圈数据 `read_multiple_coils`
    * [ ] 写单圈数据 `write_single_coil`
    * [ ] 写多圈数据 `write_coils`
    * [ ] 读离散量输入 `read_input_status`
    * [ ] 读保持寄存器 `read_holding_registers`
    * [x] 写单个寄存器 `write_single_register`
    * [ ] 写多个寄存器 `write_registers`
    * [ ] 读多个保存寄存器 `read_multiple_holding_registers`
    * [ ] 读输入寄存器 `read_input_registers`
    * [ ] 读多个输入寄存器 `read_multiple_input_registers`
* [ ] 电子围栏与虚拟指令集 `electronicFence`
  * [ ] 数据管理
    * [ ] 新增几何模型参数 `add_electronic_fence_config`
    * [ ] 更新集合模型参数 `update_electronic_fence_config`
    * [ ] 删除指定几何模型 `delete_electronic_fence_config`
    * [ ] 查询所有几何模型 `delete_electronic_fence_config`
    * [ ] 查询指定几何模型 `given_electronic_fence_config`
    * [ ] 查询所有几何模型参数 `get_electronic_fence_list_infos`
  * [ ] 电子围栏
    * [ ] 设置电子围栏使能状态 `set_electronic_fence_enable`
    * [ ] 获取电子围栏使能状态 `get_electronic_fence_enable`
    * [ ] 设置当前电子围栏参数 `set_electronic_fence_config`
    * [ ] 获取当前电子围栏参数 `get_electronic_fence_config`
  * [ ] 虚拟墙
    * [ ] 设置虚拟墙使能状态 `set_virtual_wall_enable`
    * [ ] 获取虚拟墙使能状态 `get_virtual_wall_enable`
    * [ ] 设置当前虚拟墙参数 `set_virtual_wall_config`
    * [ ] 获取当前虚拟墙参数 `get_virtual_wall_config`
* [ ] 力传感器指令集（选配） `forceSensor`
  * [ ] 六维力
    * [ ] 查询六维力数据 `get_force_data`
    * [ ] 六维力数据清零 `clear_force_data`
    * [ ] 自动设置六维力重心参数 `set_force_sensor`
    * [ ] 手动标定六维力数据 `manual_set_force`
    * [ ] 停止标定力传感器重心 `stop_set_force_sensor`
  * [ ] 一维力
    * [ ] 查询末端一维力数据 `get_Fz`
    * [ ] 清零末端一维力数据 `clear_Fz`
    * [ ] 自动标定一维力数据 `auto_set_Fz`
    * [ ] 手动标定一维力数据 `manual_set_Fz`
    * [ ] 停止标定力传感器重心 `stop_set_force_sensor`
* [ ] 末端工具指令集（选配） `endTool`
  * [ ] 夹爪控制
    * [ ] 设置夹爪行程 `set_gripper_route`
    * [ ] 松开夹爪 `set_gripper_release`
    * [ ] 夹爪力控夹取 `set_gripper_pick`
    * [ ] 夹爪持续力控夹取 `set_gripper_pick_on`
    * [ ] 夹爪到达指定位置 `set_gripper_position`
    * [ ] 查询夹爪状态 `get_gripper_state`
  * [ ] 五指灵巧手
    * [ ] 设置灵巧手手势 `set_hand_posture`
    * [ ] 设置灵巧手动作序列 `set_hand_seq`
    * [ ] 设置灵巧手各自由度角度 `set_hand_angle`
    * [ ] 设置灵巧手速度 `set_hand_speed`
    * [ ] 设置灵巧手力阈值 `set_hand_force`
    * [ ] 灵巧手角度跟随控制 `hand_follow_angle`
    * [ ] 灵巧手位置跟随控制 `hand_follow_pos`
* [ ] 控制器扩展设备指令集 `expandControl`
  * [ ] 升降机构
    * [ ] 速度开环控制 `set_lift_speed`
    * [ ] 位置闭环控制 `set_lift_height`
    * [ ] 获取升降机构状态 `get_lift_state`
  * [ ] 通用扩展关节
    * [ ] 扩展关节状态获取 `expand_get_state`
    * [ ] 关节速度环控制 `expand_set_speed`
    * [ ] 关节位置环控制 `expand_set_pos`
* [ ] 附录：错误代码 `errorList`
  * [ ] 系统错误代码
  * [ ] 关节错误代码
