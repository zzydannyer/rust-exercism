# Instructions

# 指示

Your task is to create a program that implements the Sieve of Eratosthenes algorithm to find all prime numbers less than or equal to a given number.

您的任务是创建一个程序来实现埃拉托斯特尼筛法算法，以查找所有小于或等于给定数字的素数。

A prime number is a number larger than 1 that is only divisible by 1 and itself.
For example, 2, 3, 5, 7, 11, and 13 are prime numbers.
By contrast, 6 is _not_ a prime number as it not only divisible by 1 and itself, but also by 2 and 3.

素数是大于 1 且只能被 1 和它本身整除的数。
例如，2、3、5、7、11 和 13 是质数。
相比之下，6 并不是素数，因为它不仅能被 1 和它本身整除，还能被 2 和 3 整除。

To use the Sieve of Eratosthenes, first, write out all the numbers from 2 up to and including your given number.
Then, follow these steps:

要使用埃拉托斯特尼筛法，首先写出从 2 到（包括）给定数字的所有数字。
然后，按照下列步骤操作：

1. Find the next unmarked number (skipping over marked numbers).
   This is a prime number.
2. Mark all the multiples of that prime number as **not** prime.

1. 找到下一个未标记的数字（跳过标记的数字）。
   这是一个质数。
2. 将该素数的所有倍数标记为**非**素数。

Repeat the steps until you've gone through every number.
At the end, all the unmarked numbers are prime.

重复这些步骤，直到完成所有数字。
最后，所有未标记的数字都是质数。

~~~~exercism/note
The Sieve of Eratosthenes marks off multiples of each prime using addition (repeatedly adding the prime) or multiplication (directly computing its multiples), rather than checking each number for divisibility.

~~~~练习/注意事项
埃拉托斯特尼筛法使用加法（重复添加素数）或乘法（直接计算其倍数）来标记每个素数的倍数，而不是检查每个数字的整除性。

The tests don't check that you've implemented the algorithm, only that you've come up with the correct primes.
~~~~

测试不会检查您是否已实现该算法，只是检查您是否得出了正确的素数。
～～～

## Example

## 例子

Let's say you're finding the primes less than or equal to 10.

假设您要查找小于或等于 10 的素数。

- Write out 2, 3, 4, 5, 6, 7, 8, 9, 10, leaving them all unmarked.

- 写出 2、3、4、5、6、7、8、9、10，不做任何标记。

  ```text
  2 3 4 5 6 7 8 9 10
  ```

- 2 is unmarked and is therefore a prime.
  Mark 4, 6, 8 and 10 as "not prime".

- 2 没有标记，因此是质数。
  将 4、6、8 和 10 标记为“非质数”。

  ```text
  2 3 [4] 5 [6] 7 [8] 9 [10]
  ↑
  ```

- 3 is unmarked and is therefore a prime.
  Mark 6 and 9 as not prime _(marking 6 is optional - as it's already been marked)_.

- 3 没有标记，因此是素数。
  将 6 和 9 标记为非质数_（标记 6 是可选的 - 因为它已经被标记）_。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
    ↑
  ```

- 4 is marked as "not prime", so we skip over it.

- 4 被标记为“非素数”，因此我们跳过它。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
       ↑
  ```

- 5 is unmarked and is therefore a prime.
  Mark 10 as not prime _(optional - as it's already been marked)_.

- 5 没有标记，因此是素数。
  将 10 标记为非素数_（可选 - 因为它已经被标记）_。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
          ↑
  ```

- 6 is marked as "not prime", so we skip over it.

- 6 被标记为“非素数”，因此我们跳过它。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
             ↑
  ```

- 7 is unmarked and is therefore a prime.

- 7 没有标记，因此是素数。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
                ↑
  ```

- 8 is marked as "not prime", so we skip over it.

- 8 被标记为“非素数”，因此我们跳过它。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
                   ↑
  ```

- 9 is marked as "not prime", so we skip over it.

- 9 被标记为“非素数”，所以我们跳过它。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
                       ↑
  ```

- 10 is marked as "not prime", so we stop as there are no more numbers to check.

- 10 被标记为“非质数”，因此我们停止，因为没有更多数字可供检查。

  ```text
  2 3 [4] 5 [6] 7 [8] [9] [10]
                           ↑
  ```

You've examined all the numbers and found that 2, 3, 5, and 7 are still unmarked, meaning they're the primes less than or equal to 10.


您检查了所有数字，发现 2、3、5 和 7 仍然没有标记，这意味着它们是小于或等于 10 的素数。