# Instructions

# 指示

Your task is to find the potential trees where you could build your tree house.

你的任务是找到可以建造树屋的潜在树木。

The data company provides the data as grids that show the heights of the trees.
The rows of the grid represent the east-west direction, and the columns represent the north-south direction.

数据公司以网格形式提供数据，显示树木的高度。
网格的行代表东西方向，列代表南北方向。

An acceptable tree will be the largest in its row, while being the smallest in its column.

可接受的树将是其行中最大的树，而其列中最小的树。

A grid might not have any good trees at all.
Or it might have one, or even several.

网格可能根本没有任何好树。
或者它可能有一个，甚至多个。

Here is a grid that has exactly one candidate tree.

这是一个只有一棵候选树的网格。

```text
      ↓
      1  2  3  4
    |-----------
  1 | 9  8  7  8
→ 2 |[5] 3  2  4
  3 | 6  6  7  1
```

- Row 2 has values 5, 3, 2, and 4. The largest value is 5.
- Column 1 has values 9, 5, and 6. The smallest value is 5.

- 第 2 行的值为 5、3、2 和 4。最大值为 5。
- 第 1 列的值为 9、5 和 6。最小值为 5。

So the point at `[2, 1]` (row: 2, column: 1) is a great spot for a tree house.


因此，`[2, 1]`（行：2，列：1）处的点是树屋的好地方。