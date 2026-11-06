# Description

# 描述

Today, most people in the world use Arabic numerals (0–9).
But if you travelled back two thousand years, you'd find that most Europeans were using Roman numerals instead.

今天，世界上大多数人使用阿拉伯数字（0-9）。
但如果你回到两千多年前，你会发现大多数欧洲人都在使用罗马数字。

To write a Roman numeral we use the following Latin letters, each of which has a value:

为了书写罗马数字，我们使用以下拉丁字母，每个字母都有一个值：

| M    | D   | C   | L   | X   | V   | I   |
| ---- | --- | --- | --- | --- | --- | --- |
| 1000 | 500 | 100 | 50  | 10  | 5   | 1   |

A Roman numeral is a sequence of these letters, and its value is the sum of the letters' values.
For example, `XVIII` has the value 18 (`10 + 5 + 1 + 1 + 1 = 18`).

罗马数字是这些字母的序列，其值是字母值的总和。
例如，`XVIII` 的值为 18 (`10 + 5 + 1 + 1 + 1 = 18`)。

There's one rule that makes things trickier though, and that's that **the same letter cannot be used more than three times in succession**.
That means that we can't express numbers such as 4 with the seemingly natural `IIII`.
Instead, for those numbers, we use a subtraction method between two letters.
So we think of `4` not as `1 + 1 + 1 + 1` but instead as `5 - 1`.
And slightly confusingly to our modern thinking, we write the smaller number first.
This applies only in the following cases: 4 (`IV`), 9 (`IX`), 40 (`XL`), 90 (`XC`), 400 (`CD`) and 900 (`CM`).

但有一条规则让事情变得更加棘手，那就是**同一个字母不能连续使用超过三次**。
这意味着我们无法用看似自然的 `IIII` 来表达诸如 4 之类的数字。
相反，对于这些数字，我们使用两个字母之间的减法。
因此，我们将 `4` 视为 `5 - 1`，而不是 `1 + 1 + 1 + 1`。
对于我们的现代思维来说有点令人困惑的是，我们先写较小的数字。
这仅适用于以下情况：4 (`IV`)、9 (`IX`)、40 (`XL`)、90 (`XC`)、400 (`CD`) 和 900 (`CM`)。

Order matters in Roman numerals!
Letters (and the special compounds above) must be ordered by decreasing value from left to right.

罗马数字的顺序很重要！
字母（以及上面的特殊组合）必须按从左到右递减的值排序。

Here are some examples:

以下是一些示例：

```text
 105 => CV
---- => --
 100 => C
+  5 =>  V
```

```text
 106 => CVI
---- => --
 100 => C
+  5 =>  V
+  1 =>   I
```

```text
 104 => CIV
---- => ---
 100 => C
+  4 =>  IV
```

And a final more complex example:

最后一个更复杂的例子：

```text
 1996 => MCMXCVI
----- => -------
 1000 => M
+ 900 =>  CM
+  90 =>    XC
+   5 =>      V
+   1 =>       I
```
