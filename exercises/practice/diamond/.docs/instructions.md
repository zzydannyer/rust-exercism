# Instructions

# 指示

The diamond kata takes as its input a letter, and outputs it in a diamond shape.
Given a letter, it prints a diamond starting with 'A', with the supplied letter at the widest point.

菱形型将字母作为输入，并以菱形输出。
给定一个字母，它会打印一个以“A”开头的菱形，提供的字母位于最宽的点。

## Requirements

## 要求

- The first row contains one 'A'.
- The last row contains one 'A'.
- All rows, except the first and last, have exactly two identical letters.
- All rows have as many trailing spaces as leading spaces. (This might be 0).
- The diamond is horizontally symmetric.
- The diamond is vertically symmetric.
- The diamond has a square shape (width equals height).
- The letters form a diamond shape.
- The top half has the letters in ascending order.
- The bottom half has the letters in descending order.
- The four corners (containing the spaces) are triangles.

- 第一行包含一个“A”。
- 最后一行包含一个“A”。
- 除第一行和最后一行外，所有行都有两个完全相同的字母。
- 所有行的尾随空格与前导空格一样多。 （这可能是 0）。
- 钻石水平对称。
- 钻石垂直对称。
- 钻石呈方形（宽度等于高度）。
- 字母形成菱形。
- 上半部分的字母按升序排列。
- 下半部分的字母按降序排列。
- 四个角（包含空格）是三角形。

## Examples

## 示例

In the following examples, spaces are indicated by `·` characters.

在以下示例中，空格由 `·` 字符表示。

Diamond for letter 'A':

字母“A”的菱形：

```text
A
```

Diamond for letter 'C':

字母“C”的菱形：

```text
··A··
·B·B·
C···C
·B·B·
··A··
```

Diamond for letter 'E':

字母“E”的菱形：

```text
····A····
···B·B···
··C···C··
·D·····D·
E·······E
·D·····D·
··C···C··
···B·B···
····A····
```
