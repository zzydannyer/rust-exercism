# Instructions

# 指示

Given the position of two queens on a chess board, indicate whether or not they are positioned so that they can attack each other.

给定两个皇后在棋盘上的位置，指出它们的位置是否可以互相攻击。

In the game of chess, a queen can attack pieces which are on the same row, column, or diagonal.

在国际象棋游戏中，皇后可以攻击同一行、同一列或对角线上的棋子。

A chessboard can be represented by an 8 by 8 array.

棋盘可以用 8 x 8 数组表示。

So if you are told the white queen is at `c5` (zero-indexed at column 2, row 3) and the black queen at `f2` (zero-indexed at column 5, row 6), then you know that the set-up is like so:

因此，如果您被告知白皇后位于 `c5`（零索引位于第 2 列第 3 行），而黑皇后位于 `f2`（零索引位于第 5 列第 6 行），那么您就知道设置如下：

![A chess board with two queens. Arrows emanating from the queen at c5 indicate possible directions of capture along file, rank and diagonal.](https://assets.exercism.org/images/exercises/queen-attack/queen-capture.svg)

!ZXQ保护0QXZ

You are also able to answer whether the queens can attack each other.
In this case, that answer would be yes, they can, because both pieces share a diagonal.

您还可以回答皇后是否可以互相攻击。
在这种情况下，答案是肯定的，他们可以，因为两个部分共享一条对角线。

## Credit

## 信用

The chessboard image was made by [habere-et-dispertire][habere-et-dispertire] using LaTeX and the [chessboard package][chessboard-package] by Ulrike Fischer.

棋盘图像是由 [habere-et-分散剂][habere-et-分散剂] 使用 LaTeX 和 Ulrike Fischer 的 [棋盘包][棋盘包] 制作的。

[habere-et-dispertire]: https://exercism.org/profiles/habere-et-dispertire
[chessboard-package]: https://github.com/u-fischer/chessboard


[哈贝尔和分散]：https://exercism.org/profiles/habere-et-dispertire
[棋盘包]：https://github.com/u-fischer/chessboard