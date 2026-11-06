# Instructions

# 指示

Determine if a triangle is equilateral, isosceles, or scalene.

确定三角形是等边三角形、等腰三角形还是不等边三角形。

An _equilateral_ triangle has all three sides the same length.

_等边_三角形的所有三个边的长度都相同。

An _isosceles_ triangle has at least two sides the same length.
(It is sometimes specified as having exactly two sides the same length, but for the purposes of this exercise we'll say at least two.)

等腰三角形至少有两条边长度相同。
（有时指定为具有完全相同长度的两条边，但出于本练习的目的，我们会说至少两条边。）

A _scalene_ triangle has all sides of different lengths.

_scalene_ 三角形的所有边的长度都不同。

## Note

## 笔记

For a shape to be a triangle at all, all sides have to be of length > 0, and the sum of the lengths of any two sides must be greater than or equal to the length of the third side.

要使形状成为三角形，所有边的长度必须 > 0，并且任何两条边的长度之和必须大于或等于第三条边的长度。

~~~~exercism/note
_Degenerate triangles_ are triangles where the sum of the length of two sides is **equal** to the length of the third side, e.g. `1, 1, 2`.
We opted to not include tests for degenerate triangles in this exercise.
You may handle those situations if you wish to do so, or safely ignore them.
~~~~

~~~~练习/注意事项
_退化三角形_是两条边的长度之和**等于**第三条边的长度的三角形，例如ZXQ保护0QXZ。
我们选择在此练习中不包括对退化三角形的测试。
如果您愿意，您可以处理这些情况，或者安全地忽略它们。
～～～

In equations:

在方程中：

Let `a`, `b`, and `c` be sides of the triangle.
Then all three of the following expressions must be true:

令 `a`、`b` 和 `c` 为三角形的边。
那么以下三个表达式都必须为真：

```text
a + b ≥ c
b + c ≥ a
a + c ≥ b
```

See [Triangle Inequality][triangle-inequality]

请参阅[三角形不等式][三角形不等式]

[triangle-inequality]: https://en.wikipedia.org/wiki/Triangle_inequality


[三角形不等式]：https://en.wikipedia.org/wiki/Triangle_inequality