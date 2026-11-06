# Instructions

# 指示

Score a bowling game.

保龄球比赛得分。

Bowling is a game where players roll a heavy ball to knock down pins arranged in a triangle.
Write code to keep track of the score of a game of bowling.

保龄球是一项玩家滚动重球以击倒呈三角形排列的球瓶的游戏。
编写代码来跟踪保龄球比赛的得分。

## Scoring Bowling

## 保龄球得分

The game consists of 10 frames.
A frame is composed of one or two ball throws with 10 pins standing at frame initialization.
There are three cases for the tabulation of a frame.

游戏由 10 帧组成。
一帧由一或两个球投掷组成，在帧初始化时有 10 个球瓶站立。
框架的制表存在三种情况。

- An open frame is where a score of less than 10 is recorded for the frame.
  In this case the score for the frame is the number of pins knocked down.

- 开放帧是指该帧的得分低于 10 分。
  在这种情况下，该帧的得分是击倒的瓶数。

- A spare is where all ten pins are knocked down by the second throw.
  The total value of a spare is 10 plus the number of pins knocked down in their next throw.

- 备用是指第二次投掷时所有十个瓶都被击倒。
  备用瓶的总价值为 10 加上下一次投掷中击倒的瓶数。

- A strike is where all ten pins are knocked down by the first throw.
  The total value of a strike is 10 plus the number of pins knocked down in the next two throws.
  If a strike is immediately followed by a second strike, then the value of the first strike cannot be determined until the ball is thrown one more time.

- 全中是指第一次投掷时所有十个球瓶都被击倒。
  一次击中的总价值为 10 加上接下来两次投掷中击倒的瓶数。
  如果击球后立即进行第二次击球，则只有再次投掷球才能确定第一次击球的价值。

Here is a three frame example:

这是一个三框架示例：

|  Frame 1   |  Frame 2   |     Frame 3      |
| :--------: | :--------: | :--------------: |
| X (strike) | 5/ (spare) | 9 0 (open frame) |

Frame 1 is (10 + 5 + 5) = 20

第 1 帧为 (10 + 5 + 5) = 20

Frame 2 is (5 + 5 + 9) = 19

第 2 帧为 (5 + 5 + 9) = 19

Frame 3 is (9 + 0) = 9

第 3 帧为 (9 + 0) = 9

This means the current running total is 48.

这意味着当前的运行总数为 48。

The tenth frame in the game is a special case.
If someone throws a spare or a strike then they get one or two fill balls respectively.
Fill balls exist to calculate the total of the 10th frame.
Scoring a strike or spare on the fill ball does not give the player more fill balls.
The total value of the 10th frame is the total number of pins knocked down.

游戏中的第十帧是一个特例。
如果有人投出备用球或好球，那么他们将分别获得一个或两个填充球。
填充球的存在是为了计算第 10 帧的总数。
在补球上获得好球或备用球并不会为玩家提供更多补球。
第 10 帧的总值是击倒的瓶数总数。

For a tenth frame of X1/ (strike and a spare), the total value is 20.

对于 X1/ 的第十个帧（罢工和备用），总值为 20。

For a tenth frame of XXX (three strikes), the total value is 30.

对于 XXX 的第十帧（三击），总值为 30。

## Requirements

## 要求

Write code to keep track of the score of a game of bowling.
It should support two operations:

编写代码来跟踪保龄球比赛的得分。
它应该支持两种操作：

- `roll(pins : int)` is called each time the player rolls a ball.
  The argument is the number of pins knocked down.
- `score() : int` is called only at the very end of the game.
  It returns the total score for that game.


- 每次玩家滚球时都会调用 `roll(pins : int)`。
  参数是被击倒的引脚数量。
- `score() : int` 仅在游戏最后调用。
  它返回该游戏的总得分。