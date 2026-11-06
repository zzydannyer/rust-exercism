# Instructions

# 指示

Your task is to add the mine counts to empty squares in a completed Minesweeper board.
The board itself is a rectangle composed of squares that are either empty (`' '`) or a mine (`'*'`).

您的任务是将地雷数量添加到已完成的扫雷板中的空方块中。
棋盘本身是一个由空的（`' '`）或地雷（`'*'`）的方块组成的矩形。

For each empty square, count the number of mines adjacent to it (horizontally, vertically, diagonally).
If the empty square has no adjacent mines, leave it empty.
Otherwise replace it with the adjacent mines count.

对于每个空方格，计算与其相邻的地雷数量（水平、垂直、对角线）。
如果空方格没有相邻的地雷，则将其留空。
否则用相邻的地雷数量替换它。

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
