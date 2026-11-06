# Instructions

# 指示

Convert a sequence of digits in one base, representing a number, into a sequence of digits in another base, representing the same number.

将表示一个数字的一​​个基数的数字序列转换为表示相同数字的另一个基数的数字序列。

~~~~exercism/note
Try to implement the conversion yourself.
Do not use something else to perform the conversion for you.
~~~~

~~~~练习/注意事项
尝试自己实现转换。
不要使用其他东西来为您执行转换。
～～～

## About [Positional Notation][positional-notation]

## 关于[位置表示法][位置表示法]

In positional notation, a number in base **b** can be understood as a linear combination of powers of **b**.

在位置表示法中，以 **b** 为底的数字可以理解为 **b** 的幂的线性组合。

The number 42, _in base 10_, means:

数字 42（以 10 为基数）表示：

`(4 × 10¹) + (2 × 10⁰)`

The number 101010, _in base 2_, means:

数字 101010（以 2 为基数）表示：

`(1 × 2⁵) + (0 × 2⁴) + (1 × 2³) + (0 × 2²) + (1 × 2¹) + (0 × 2⁰)`

The number 1120, _in base 3_, means:

数字 1120（以 3 为基数）表示：

`(1 × 3³) + (1 × 3²) + (2 × 3¹) + (0 × 3⁰)`

_Yes. Those three numbers above are exactly the same. Congratulations!_

_是的。上面这三个数字完全相同。恭喜！_

[positional-notation]: https://en.wikipedia.org/wiki/Positional_notation


[位置符号]：https://en.wikipedia.org/wiki/Positional_notation