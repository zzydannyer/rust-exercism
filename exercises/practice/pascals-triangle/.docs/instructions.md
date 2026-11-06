# Instructions

# 指示

Your task is to output the first N rows of Pascal's triangle.

您的任务是输出帕斯卡三角形的前 N ​​行。

[Pascal's triangle][wikipedia] is a triangular array of positive integers.

[帕斯卡三角形][维基百科]是正整数的三角形数组。

In Pascal's triangle, the number of values in a row is equal to its row number (which starts at one).
Therefore, the first row has one value, the second row has two values, and so on.

在帕斯卡三角形中，一行中的值的数量等于其行号（从 1 开始）。
因此，第一行有一个值，第二行有两个值，依此类推。

The first (topmost) row has a single value: `1`.
Subsequent rows' values are computed by adding the numbers directly to the right and left of the current position in the previous row.

第一（最上面）行有一个值：`1`。
后续行的值是通过将数字直接添加到前一行当前位置的右侧和左侧来计算的。

If the previous row does _not_ have a value to the left or right of the current position (which only happens for the leftmost and rightmost positions), treat that position's value as zero (effectively "ignoring" it in the summation).

如果前一行在当前位置的左侧或右侧没有值（这种情况仅发生在最左边和最右边的位置），则将该位置的值视为零（在求和中有效地“忽略”它）。

## Example

## 例子

Let's look at the first 5 rows of Pascal's Triangle:

让我们看一下帕斯卡三角形的前 5 行：

```text
    1
   1 1
  1 2 1
 1 3 3 1
1 4 6 4 1
```

The topmost row has one value, which is `1`.

最上面一行有一个值，即 `1`。

The leftmost and rightmost values have only one preceding position to consider, which is the position to its right respectively to its left.
With the topmost value being `1`, it follows from this that all the leftmost and rightmost values are also `1`.

最左边和最右边的值只需要考虑一个前面的位置，即其右侧和左侧的位置。
由于最上面的值为 `1`，因此所有最左边和最右边的值也是 `1`。

The other values all have two positions to consider.
For example, the fifth row's (`1 4 6 4 1`) middle value is `6`, as the values to its left and right in the preceding row are `3` and `3`:

其他值都有两个位置需要考虑。
例如，第五行 (`1 4 6 4 1`) 的中间值为 `6`，因为前一行中其左侧和右侧的值为 `3` 和 `3`：

[wikipedia]: https://en.wikipedia.org/wiki/Pascal%27s_triangle


[维基百科]：https://en.wikipedia.org/wiki/Pascal%27s_triangle