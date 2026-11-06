# Instructions

# 指示

Your task is to add flower counts to empty squares in a completed Flower Field garden.
The garden itself is a rectangle board composed of squares that are either empty (`' '`) or a flower (`'*'`).

您的任务是在已完成的花田花园中的空方块中添加花朵数量。
花园本身是一个矩形板，由空的（`' '`）或花（`'*'`）的正方形组成。

For each empty square, count the number of flowers adjacent to it (horizontally, vertically, diagonally).
If the empty square has no adjacent flowers, leave it empty.
Otherwise replace it with the count of adjacent flowers.

对于每个空方块，计算与其相邻的花朵数量（水平、垂直、对角线）。
如果空方格没有相邻的花，则将其留空。
否则用相邻花的数量替换它。

For example, you may receive a 5 x 4 board like this (empty spaces are represented here with the '·' character for display on screen):

例如，您可能会收到这样的 5 x 4 板（此处用“·”字符表示空白以在屏幕上显示）：

```text
·*·*·
··*··
··*··
·····
```

Which your code should transform into this:

你的代码应该转换成这样：

```text
1*3*1
13*31
·2*2·
·111·
```
