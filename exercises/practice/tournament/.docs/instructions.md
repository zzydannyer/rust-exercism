# Instructions

# 指示

Tally the results of a small football competition.

统计小型足球比赛的结果。

Based on an input file containing which team played against which and what the outcome was, create a file with a table like this:

根据包含哪支球队对阵哪支球队以及结果是什么的输入文件，创建一个包含如下表的文件：

```text
Team                           | MP |  W |  D |  L |  P
Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
Blithering Badgers             |  3 |  1 |  0 |  2 |  3
Courageous Californians        |  3 |  0 |  1 |  2 |  1
```

What do those abbreviations mean?

这些缩写是什么意思？

- MP: Matches Played
- W: Matches Won
- D: Matches Drawn (Tied)
- L: Matches Lost
- P: Points

- MP：已进行的比赛
- W：赢得的比赛
- D：平局（平局）
- L：输掉比赛
- P：点数

A win earns a team 3 points.
A draw earns 1.
A loss earns 0.

一场胜利可为球队赢得 3 分。
平局赚1。
亏损赚0。

The outcome is ordered by points, descending.
In case of a tie, teams are ordered alphabetically.

结果按分数降序排列。
如果出现平局，球队将按字母顺序排列。

## Input

## 输入

Your tallying program will receive input that looks like:

您的统计程序将收到如下所示的输入：

```text
Allegoric Alaskans;Blithering Badgers;win
Devastating Donkeys;Courageous Californians;draw
Devastating Donkeys;Allegoric Alaskans;win
Courageous Californians;Blithering Badgers;loss
Blithering Badgers;Devastating Donkeys;loss
Allegoric Alaskans;Courageous Californians;win
```

The result of the match refers to the first team listed.
So this line:

比赛结果以排名第一的球队为准。
所以这一行：

```text
Allegoric Alaskans;Blithering Badgers;win
```

means that the Allegoric Alaskans beat the Blithering Badgers.

意味着寓言阿拉斯加人击败了胡言乱语獾队。

This line:

这行：

```text
Courageous Californians;Blithering Badgers;loss
```

means that the Blithering Badgers beat the Courageous Californians.

意味着喋喋不休的獾队击败了勇敢的加州人队。

And this line:

还有这一行：

```text
Devastating Donkeys;Courageous Californians;draw
```

means that the Devastating Donkeys and Courageous Californians tied.


意味着毁灭性的驴子和勇敢的加州人打成平局。