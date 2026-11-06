# Instructions

# 指示

Determine whether a number is valid according to the [Luhn formula][luhn].

根据[Luhn公式][luhn]判断一个数是否有效。

The number will be provided as a string.

该数字将以字符串形式提供。

## Validating a number

## 验证数字

Strings of length 1 or less are not valid.
Spaces are allowed in the input, but they should be stripped before checking.
All other non-digit characters are disallowed.

长度为 1 或更小的字符串无效。
输入中允许有空格，但在检查之前应将其删除。
不允许使用所有其他非数字字符。

## Examples

## 示例

### Valid credit card number

### 有效的信用卡号

The number to be checked is `4539 3195 0343 6467`.

要检查的编号是`4539 3195 0343 6467`。

The first step of the Luhn algorithm is to start at the end of the number and double every second digit, beginning with the second digit from the right and moving left.

Luhn 算法的第一步是从数字末尾开始，每隔两个数字加倍，从右侧第二个数字开始并向左移动。

```text
4539 3195 0343 6467
↑ ↑  ↑ ↑  ↑ ↑  ↑ ↑  (double these)
```

If the result of doubling a digit is greater than 9, we subtract 9 from that result.
We end up with:

如果一位数字加倍的结果大于 9，我们就从该结果中减去 9。
我们最终得到：

```text
8569 6195 0383 3437
```

Finally, we sum all digits.
If the sum is evenly divisible by 10, the original number is valid.

最后，我们将所有数字相加。
如果总和能被 10 整除，则原数有效。

```text
8 + 5 + 6 + 9 + 6 + 1 + 9 + 5 + 0 + 3 + 8 + 3 + 3 + 4 + 3 + 7 = 80
```

80 is evenly divisible by 10, so number `4539 3195 0343 6467` is valid!

80 可以被 10 整除，所以数字 `4539 3195 0343 6467` 有效！

### Invalid Canadian SIN

### 加拿大 SIN 无效

The number to be checked is `066 123 478`.

要检查的编号是`066 123 478`。

We start at the end of the number and double every second digit, beginning with the second digit from the right and moving left.

我们从数字末尾开始，每隔两个数字加倍，从右侧第二个数字开始并向左移动。

```text
066 123 478
 ↑  ↑ ↑  ↑  (double these)
```

If the result of doubling a digit is greater than 9, we subtract 9 from that result.
We end up with:

如果一位数字加倍的结果大于 9，我们就从该结果中减去 9。
我们最终得到：

```text
036 226 458
```

We sum the digits:

我们对数字求和：

```text
0 + 3 + 6 + 2 + 2 + 6 + 4 + 5 + 8 = 36
```

36 is not evenly divisible by 10, so number `066 123 478` is not valid!

36 不能被 10 整除，因此数字 `066 123 478` 无效！

[luhn]: https://en.wikipedia.org/wiki/Luhn_algorithm


[卢恩]：https://en.wikipedia.org/wiki/Luhn_algorithm