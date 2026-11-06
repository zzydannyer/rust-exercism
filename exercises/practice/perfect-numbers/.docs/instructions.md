# Instructions

# 指示

Determine if a number is perfect, abundant, or deficient based on Nicomachus' (60 - 120 CE) classification scheme for positive integers.

根据尼马科斯 (Niomachus) (60 - 120 CE) 正整数分类方案确定一个数是完美的、丰富的还是不足的。

The Greek mathematician [Nicomachus][nicomachus] devised a classification scheme for positive integers, identifying each as belonging uniquely to the categories of [perfect](#perfect), [abundant](#abundant), or [deficient](#deficient) based on their [aliquot sum][aliquot-sum].
The _aliquot sum_ is defined as the sum of the factors of a number not including the number itself.
For example, the aliquot sum of `15` is `1 + 3 + 5 = 9`.

希腊数学家 [Nicomachus][nicomachus] 设计了正整数的分类方案，根据其 [等分和][等分和] 将每个正整数识别为唯一属于 [perfect](#perfect)、[abundant](#abundant) 或 [deficient](#deficient) 类别。
“等分和”定义为一个数字的因数之和，不包括该数字本身。
例如，`15` 的等分总和为 `1 + 3 + 5 = 9`。

## Perfect

## 完美的

A number is perfect when it equals its aliquot sum.
For example:

当一个数字等于它的等分总和时，它就是完美的。
例如：

- `6` is a perfect number because `1 + 2 + 3 = 6`
- `28` is a perfect number because `1 + 2 + 4 + 7 + 14 = 28`

- `6` 是一个完美数，因为 `1 + 2 + 3 = 6`
- `28` 是完美数，因为 `1 + 2 + 4 + 7 + 14 = 28`

## Abundant

##丰富

A number is abundant when it is less than its aliquot sum.
For example:

当一个数字小于其等分总和时，该数字就是丰富的。
例如：

- `12` is an abundant number because `1 + 2 + 3 + 4 + 6 = 16`
- `24` is an abundant number because `1 + 2 + 3 + 4 + 6 + 8 + 12 = 36`

- `12` 是一个丰富的数字，因为 `1 + 2 + 3 + 4 + 6 = 16`
- `24` 是一个丰富的数字，因为 `1 + 2 + 3 + 4 + 6 + 8 + 12 = 36`

## Deficient

## 不足

A number is deficient when it is greater than its aliquot sum.
For example:

当一个数字大于其等分总和时，该数字就是不足的。
例如：

- `8` is a deficient number because `1 + 2 + 4 = 7`
- Prime numbers are deficient

- `8` 是一个不足的数字，因为 `1 + 2 + 4 = 7`
- 素数不足

## Task

## 任务

Implement a way to determine whether a given number is [perfect](#perfect).
Depending on your language track, you may also need to implement a way to determine whether a given number is [abundant](#abundant) or [deficient](#deficient).

实现一种方法来确定给定数字是否为 [perfect](#perfect)。
根据您的语言轨道，您可能还需要实现一种方法来确定给定数字是 [abundant](#abundant) 还是 [deficient](#deficient)。

[nicomachus]: https://en.wikipedia.org/wiki/Nicomachus
[aliquot-sum]: https://en.wikipedia.org/wiki/Aliquot_sum


[尼马科斯]：https://en.wikipedia.org/wiki/Nicomachus
[等分总和]：https://en.wikipedia.org/wiki/Aliquot_sum