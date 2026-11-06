# Description

# 描述

FizzBuzz is a children's game of counting through the integers. For each of them, if it's divisible by three, substitute the word "fizz"; if divisible by five, substitute "buzz"; if both, say both; if neither, say the number. It is not particularly difficult to implement, though it enjoyed some popularity for a time as [a quick way to tell whether entry-level programming applicants knew how to program _at all_](https://blog.codinghorror.com/why-cant-programmers-program/).

FizzBu​​zz 是一款通过整数进行计数的儿童游戏。对于它们中的每一个，如果它能被三整除，则替换单词“fizz”；如果能被五整除，则替换为“buzz”；如果两者都说，就说两者；如果两者都没有，请说出数字。尽管它曾一度被称为 [a quick way to tell whether entry-level programming applicants knew how to program _at all_](https://blog.codinghorror.com/why-cant-programmers-program/)，但实施起来并不是特别困难。

It has since fallen somewhat into disfavor for that task, because applicants began memorizing FizzBuzz implementations instead of learning to program.

此后，这项任务在某种程度上变得不受欢迎，因为申请人开始记住 FizzBu​​zz 的实现，而不是学习编程。

We're going to do something more interesting than the basics: your task in this exercise is to implement FizzBuzz:

我们将做一些比基础知识更有趣的事情：您在本练习中的任务是实现 FizzBu​​zz：

- with fully-customizable rules about what numbers produce what words
- fully generic on a very restricted minimal trait set
- such that it works just as well for the Collatz Sequence as for steadily increasing numbers
- with convenient helpers to make its use ergonomic


- 关于什么数字产生什么单词的完全可定制的规则
- 在非常有限的最小特征集上完全通用
- 这样一来，它对于 Collatz 序列和稳定增加的数字都同样有效
- 配有方便的助手，使其使用符合人体工学