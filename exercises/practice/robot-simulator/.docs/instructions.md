# Instructions

# 指示

Write a robot simulator.

编写一个机器人模拟器。

A robot factory's test facility needs a program to verify robot movements.

机器人工厂的测试设施需要一个程序来验证机器人的运动。

The robots have three possible movements:

机器人有三种可能的运动：

- turn right
- turn left
- advance

- 右转
- 左转
- 提前

Robots are placed on a hypothetical infinite grid, facing a particular direction (north, east, south, or west) at a set of {x,y} coordinates,
e.g., {3,8}, with coordinates increasing to the north and east.

机器人被放置在一个假设的无限网格上，在一组 {x,y} 坐标处面向特定方向（北、东、南或西），
例如，{3,8}，坐标向北和向东递增。

The robot then receives a number of instructions, at which point the testing facility verifies the robot's new position, and in which direction it is pointing.

然后，机器人会收到许多指令，此时测试设备会验证机器人的新位置及其指向的方向。

- The letter-string "RAALAL" means:
  - Turn right
  - Advance twice
  - Turn left
  - Advance once
  - Turn left yet again
- Say a robot starts at {7, 3} facing north.
  Then running this stream of instructions should leave it at {9, 4} facing west.


- 字母串“RAALAL”的含义是：
  - 右转
  - 前进两次
  - 左转
  - 前进一次
  - 再次左转
- 假设机器人从 {7, 3} 开始，面朝北。
  然后运行该指令流应使其位于面向西的 {9, 4}。