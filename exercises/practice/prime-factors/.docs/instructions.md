# Instructions

# 指示

Compute the prime factors of a given natural number.

计算给定自然数的素因数。

A prime number is only evenly divisible by itself and 1.

素数只能被它本身和 1 整除。

Note that 1 is not a prime number.

请注意，1 不是素数。

## Example

## 例子

What are the prime factors of 60?

60的主要因数有哪些？

- Our first divisor is 2.
  2 goes into 60, leaving 30.
- 2 goes into 30, leaving 15.
  - 2 doesn't go cleanly into 15.
    So let's move on to our next divisor, 3.
- 3 goes cleanly into 15, leaving 5.
  - 3 does not go cleanly into 5.
    The next possible factor is 4.
  - 4 does not go cleanly into 5.
    The next possible factor is 5.
- 5 does go cleanly into 5.
- We're left only with 1, so now, we're done.

Our successful divisors in that computation represent the list of prime factors of 60: 2, 2, 3, and 5.

我们在该计算中成功的除数代表了 60 的质因数列表：2、2、3 和 5。

You can check this yourself:

你可以自己检查一下：

```text
2 * 2 * 3 * 5
= 4 * 15
= 60
```

Success!


成功！