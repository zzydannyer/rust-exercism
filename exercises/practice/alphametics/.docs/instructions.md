# Instructions

# 指示

Given an alphametics puzzle, find the correct solution.

给定一个字母算术难题，找到正确的解决方案。

[Alphametics][alphametics] is a puzzle where letters in words are replaced with numbers.

[Alphametics][alphametics] 是一个将单词中的字母替换为数字的谜题。

For example `SEND + MORE = MONEY`:

例如`SEND + MORE = MONEY`：

```text
  S E N D
  M O R E +
-----------
M O N E Y
```

Replacing these with valid numbers gives:

将这些替换为有效数字可以得到：

```text
  9 5 6 7
  1 0 8 5 +
-----------
1 0 6 5 2
```

This is correct because every letter is replaced by a different number and the words, translated into numbers, then make a valid sum.

这是正确的，因为每个字母都被替换为不同的数字，并且单词被翻译成数字，然后得出有效的总和。

Each letter must represent a different digit, and the leading digit of a multi-digit number must not be zero.

每个字母必须代表不同的数字，多位数字的前导数字不能为零。

[alphametics]: https://en.wikipedia.org/wiki/Alphametics


[字母学]：https://en.wikipedia.org/wiki/Alphametics