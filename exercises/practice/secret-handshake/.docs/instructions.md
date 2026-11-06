# Instructions

# 指示

Your task is to convert a number between 1 and 31 to a sequence of actions in the secret handshake.

您的任务是将 1 到 31 之间的数字转换为秘密握手中的一系列操作。

The sequence of actions is chosen by looking at the rightmost five digits of the number once it's been converted to binary.
Start at the right-most digit and move left.

操作顺序是通过查看转换为二进制后的数字最右边的五位数字来选择的。
从最右边的数字开始并向左移动。

The actions for each number place are:

每个数字位置的操作是：

```plaintext
00001 = wink
00010 = double blink
00100 = close your eyes
01000 = jump
10000 = Reverse the order of the operations in the secret handshake.
```

Let's use the number `9` as an example:

我们以号码 `9` 为例：

- 9 in binary is `1001`.
- The digit that is farthest to the right is 1, so the first action is `wink`.
- Going left, the next digit is 0, so there is no double-blink.
- Going left again, the next digit is 0, so you leave your eyes open.
- Going left again, the next digit is 1, so you jump.

- 二进制中的 9 是 `1001`。
- 最右边的数字是1，所以第一个动作是`wink`。
- 向左走，下一个数字是 0，所以没有双闪。
- 再向左走，下一个数字是 0，所以你睁开眼睛。
- 再向左走，下一个数字是 1，所以你跳了。

That was the last digit, so the final code is:

这是最后一位数字，所以最终的代码是：

```plaintext
wink, jump
```

Given the number 26, which is `11010` in binary, we get the following actions:

给定数字 26，即二进制的 `11010`，我们得到以下操作：

- double blink
- jump
- reverse actions

- 两次眨眼
- 跳跃
- 反向动作

The secret handshake for 26 is therefore:

因此 26 的秘密握手是：

```plaintext
jump, double blink
```

~~~~exercism/note
If you aren't sure what binary is or how it works, check out [this binary tutorial][intro-to-binary].

~~~~练习/注意事项
如果您不确定二进制是什么或它如何工作，请查看[此二进制教程][二进制简介]。

[intro-to-binary]: https://medium.com/basecs/bits-bytes-building-with-binary-13cb4289aafa
~~~~


[二进制介绍]：https://medium.com/basecs/bits-bytes-building-with-binary-13cb4289aafa
～～～