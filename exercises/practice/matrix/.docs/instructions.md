# Instructions

# 指示

Given a string representing a matrix of numbers, return the rows and columns of that matrix.

给定一个表示数字矩阵的字符串，返回该矩阵的行和列。

So given a string with embedded newlines like:

因此，给定一个嵌入换行符的字符串，例如：

```text
9 8 7
5 3 2
6 6 7
```

representing this matrix:

表示这个矩阵：

```text
    1  2  3
  |---------
1 | 9  8  7
2 | 5  3  2
3 | 6  6  7
```

your code should be able to spit out:

你的代码应该能够输出：

- A list of the rows, reading each row left-to-right while moving top-to-bottom across the rows,
- A list of the columns, reading each column top-to-bottom while moving from left-to-right.

- 行列表，从左到右读取每行，同时从上到下跨行移动，
- 列列表，从左到右移动时从上到下阅读每列。

The rows for our example matrix:

我们的示例矩阵的行：

- 9, 8, 7
- 5, 3, 2
- 6, 6, 7

And its columns:

及其专栏：

- 9, 5, 6
- 8, 3, 6
- 7, 2, 7
