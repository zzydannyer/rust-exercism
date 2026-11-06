# Instructions

# 指示

Implement an evaluator for a very simple subset of Forth.

为 Forth 的一个非常简单的子集实现一个评估器。

[Forth][forth]
is a stack-based programming language.
Implement a very basic evaluator for a small subset of Forth.

[第四][第四]
是一种基于堆栈的编程语言。
为 Forth 的一小部分实现一个非常基本的评估器。

Your evaluator has to support the following words:

您的评估员必须支持以下词语：

- `+`, `-`, `*`, `/` (integer arithmetic)
- `DUP`, `DROP`, `SWAP`, `OVER` (stack manipulation)

- `+`、`-`、`*`、`/`（整数运算）
- `DUP`、`DROP`、`SWAP`、`OVER`（堆栈操作）

Your evaluator also has to support defining new words using the customary syntax: `: word-name definition ;`.

您的评估器还必须支持使用惯用语法定义新单词：`: word-name definition ;`。

To keep things simple the only data type you need to support is signed integers of at least 16 bits size.

为了简单起见，您需要支持的唯一数据类型是至少 16 位大小的有符号整数。

You should use the following rules for the syntax: a number is a sequence of one or more (ASCII) digits, a word is a sequence of one or more letters, digits, symbols or punctuation that is not a number.
(Forth probably uses slightly different rules, but this is close enough.)

您应该使用以下语法规则：数字是一个或多个 (ASCII) 数字的序列，单词是一个或多个字母、数字、符号或非数字标点符号的序列。
（Forth 可能使用稍微不同的规则，但这已经足够接近了。）

Words are case-insensitive.

单词不区分大小写。

[forth]: https://en.wikipedia.org/wiki/Forth_%28programming_language%29


[第四]：https://en.wikipedia.org/wiki/Forth_%28programming_language%29