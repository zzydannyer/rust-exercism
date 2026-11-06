# Instructions

# 指示

The [ISBN-10 verification process][isbn-verification] is used to validate book identification numbers.
These normally contain dashes and look like: `3-598-21508-8`

[ISBN-10 验证过程][isbn-verification] 用于验证图书标识号。
这些通常包含破折号，看起来像：`3-598-21508-8`

## ISBN

## 国际标准书号

The ISBN-10 format is 9 digits (0 to 9) plus one check character (either a digit or an X only).
In the case the check character is an X, this represents the value '10'.
These may be communicated with or without hyphens, and can be checked for their validity by the following formula:

ISBN-10 格式为 9 位数字（0 到 9）加一个校验字符（数字或仅 X）。
如果校验字符是 X，则表示值“10”。
这些可以使用连字符或不使用连字符来表示，并且可以通过以下公式检查其有效性：

```text
(d₁ * 10 + d₂ * 9 + d₃ * 8 + d₄ * 7 + d₅ * 6 + d₆ * 5 + d₇ * 4 + d₈ * 3 + d₉ * 2 + d₁₀ * 1) mod 11 == 0
```

If the result is 0, then it is a valid ISBN-10, otherwise it is invalid.

如果结果为0，则它​​是有效的ISBN-10，否则它是无效的。

## Example

## 例子

Let's take the ISBN-10 `3-598-21508-8`.
We plug it in to the formula, and get:

我们以 ISBN-10 `3-598-21508-8` 为例。
我们将其代入公式，得到：

```text
(3 * 10 + 5 * 9 + 9 * 8 + 8 * 7 + 2 * 6 + 1 * 5 + 5 * 4 + 0 * 3 + 8 * 2 + 8 * 1) mod 11 == 0
```

Since the result is 0, this proves that our ISBN is valid.

由于结果是0，这证明我们的ISBN是有效的。

## Task

## 任务

Given a string the program should check if the provided string is a valid ISBN-10.
Putting this into place requires some thinking about preprocessing/parsing of the string prior to calculating the check digit for the ISBN.

给定一个字符串，程序应该检查提供的字符串是否是有效的 ISBN-10。
要实现这一点，需要在计算 ISBN 的校验位之前考虑对字符串进行预处理/解析。

The program should be able to verify ISBN-10 both with and without separating dashes.

该程序应该能够验证带有或不带有分隔破折号的 ISBN-10。

## Caveats

## 注意事项

Converting from strings to numbers can be tricky in certain languages.
Now, it's even trickier since the check digit of an ISBN-10 may be 'X' (representing '10').
For instance `3-598-21507-X` is a valid ISBN-10.

在某些语言中，从字符串转换为数字可能很棘手。
现在，事情变得更加棘手，因为 ISBN-10 的校验位可能是“X”（代表“10”）。
例如，`3-598-21507-X` 是有效的 ISBN-10。

[isbn-verification]: https://en.wikipedia.org/wiki/International_Standard_Book_Number


[isbn-验证]：https://en.wikipedia.org/wiki/International_Standard_Book_Number