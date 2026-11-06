# Instructions

# 指示

Parse and evaluate simple math word problems returning the answer as an integer.

解析和评估简单的数学应用题，以整数形式返回答案。

## Iteration 0 — Numbers

## 迭代 0 — 数字

Problems with no operations simply evaluate to the number given.

没有任何操作的问题只是简单地评估给定的数字。

> What is 5?

> 5 是什么？

Evaluates to 5.

评价为5。

## Iteration 1 — Addition

## 迭代 1 — 加法

Add two numbers together.

将两个数字相加。

> What is 5 plus 13?

> 5加13等于多少？

Evaluates to 18.

评估为18。

Handle large numbers and negative numbers.

处理大数和负数。

## Iteration 2 — Subtraction, Multiplication and Division

## 迭代 2 — 减法、乘法和除法

Now, perform the other three operations.

现在，执行其他三个操作。

> What is 7 minus 5?

> 7 减 5 是多少？

2

> What is 6 multiplied by 4?

> 6 乘以 4 等于多少？

24

> What is 25 divided by 5?

> 25 除以 5 等于多少？

5

## Iteration 3 — Multiple Operations

## 迭代 3 — 多次操作

Handle a set of operations, in sequence.

按顺序处理一组操作。

Since these are verbal word problems, evaluate the expression from left-to-right, _ignoring the typical order of operations._

由于这些是言语应用问题，请从左到右评估表达式，_忽略典型的运算顺序。_

> What is 5 plus 13 plus 6?

> 5加13加6等于多少？

24

> What is 3 plus 2 multiplied by 3?

> 3 加 2 乘以 3 等于多少？

15 (i.e. not 9)

15（即不是 9）

## Iteration 4 — Errors

## 迭代 4 — 错误

The parser should reject:

解析器应该拒绝：

- Unsupported operations ("What is 52 cubed?")
- Non-math questions ("Who is the President of the United States")
- Word problems with invalid syntax ("What is 1 plus plus 2?")


- 不支持的运算（“52 的立方是多少？”）
- 非数学问题（“谁是美国总统”）
- 语法无效的单词问题（“什么是 1 加加 2？”）