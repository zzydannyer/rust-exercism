# Instructions

# 指示

Your task is to change the data format of letters and their point values in the game.

您的任务是更改游戏中字母的数据格式及其点值。

Currently, letters are stored in groups based on their score, in a one-to-many mapping.

目前，字母根据其分数以一对多映射的形式存储在组中。

- 1 point: "A", "E", "I", "O", "U", "L", "N", "R", "S", "T",
- 2 points: "D", "G",
- 3 points: "B", "C", "M", "P",
- 4 points: "F", "H", "V", "W", "Y",
- 5 points: "K",
- 8 points: "J", "X",
- 10 points: "Q", "Z",

- 1分：“A”、“E”、“I”、“O”、“U”、“L”、“N”、“R”、“S”、“T”、
- 2分：“D”、“G”、
- 3分：“B”、“C”、“M”、“P”、
- 4点：“F”、“H”、“V”、“W”、“Y”、
- 5分：“K”，
- 8 分：“J”、“X”、
- 10分：“Q”，“Z”，

This needs to be changed to store each individual letter with its score in a one-to-one mapping.

需要对此进行更改，以将每个单独的字母及其分数存储在一对一的映射中。

- "a" is worth 1 point.
- "b" is worth 3 points.
- "c" is worth 3 points.
- "d" is worth 2 points.
- etc.

- “a”值 1 分。
- “b”值 3 分。
- “c”值 3 分。
- “d”值 2 分。
- ETC。

As part of this change, the team has also decided to change the letters to be lower-case rather than upper-case.

作为此更改的一部分，团队还决定将字母更改为小写而不是大写。

~~~~exercism/note
If you want to look at how the data was previously structured and how it needs to change, take a look at the examples in the test suite.
~~~~


~~~~练习/注意事项
如果您想了解数据以前的结构以及需要如何更改，请查看测试套件中的示例。
～～～