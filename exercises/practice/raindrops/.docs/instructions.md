# Instructions

# 指示

Your task is to convert a number into its corresponding raindrop sounds.

您的任务是将数字转换为其相应的雨滴声音。

If a given number:

如果给定数字：

- is divisible by 3, add "Pling" to the result.
- is divisible by 5, add "Plang" to the result.
- is divisible by 7, add "Plong" to the result.
- **is not** divisible by 3, 5, or 7, the result should be the number as a string.

- 能被3整除，在结果中添加“Pling”。
- 能被5整除，在结果中添加“Plang”。
- 能被7整除，在结果中添加“Plong”。
- **不能**被 3、5 或 7 整除，结果应该是字符串形式的数字。

## Examples

## 示例

- 28 is divisible by 7, but not 3 or 5, so the result would be `"Plong"`.
- 30 is divisible by 3 and 5, but not 7, so the result would be `"PlingPlang"`.
- 34 is not divisible by 3, 5, or 7, so the result would be `"34"`.

- 28 可以被 7 整除，但不能被 3 或 5 整除，因此结果将是 `"Plong"`。
- 30 可以被 3 和 5 整除，但不能被 7 整除，因此结果将是 `"PlingPlang"`。
- 34 不能被 3、5 或 7 整除，因此结果将为 `"34"`。

~~~~exercism/note
A common way to test if one number is evenly divisible by another is to compare the [remainder][remainder] or [modulus][modulo] to zero.
Most languages provide operators or functions for one (or both) of these.

~~~~练习/注意事项
测试一个数字是否能被另一个数字整除的常见方法是将[余数][余数]或[模][模]与零进行比较。
大多数语言为其中之一（或两者）提供运算符或函数。

[remainder]: https://exercism.org/docs/programming/operators/remainder
[modulo]: https://en.wikipedia.org/wiki/Modulo_operation
~~~~


【余数】：https://exercism.org/docs/programming/operators/remainder
[取模]：https://en.wikipedia.org/wiki/Modulo_operation
～～～