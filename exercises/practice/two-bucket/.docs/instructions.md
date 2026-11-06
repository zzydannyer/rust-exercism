# Instructions

# 指示

Given two buckets of different size and which bucket to fill first, determine how many actions are required to measure an exact number of liters by strategically transferring fluid between the buckets.

给定两个不同尺寸的桶以及首先填充哪个桶，通过策略性地在桶之间转移液体来确定需要多少次操作来测量准确的升数。

There are some rules that your solution must follow:

您的解决方案必须遵循一些规则：

- You can only do one action at a time.
- There are only 3 possible actions:
  1. Pouring one bucket into the other bucket until either:
     a) the first bucket is empty
     b) the second bucket is full
  2. Emptying a bucket and doing nothing to the other.
  3. Filling a bucket and doing nothing to the other.
- After an action, you may not arrive at a state where the initial starting bucket is empty and the other bucket is full.

Your program will take as input:

您的程序将作为输入：

- the size of bucket one
- the size of bucket two
- the desired number of liters to reach
- which bucket to fill first, either bucket one or bucket two

- 桶一的大小
- 桶二的大小
- 想要达到的升数
- 首先填充哪个桶，桶一或桶二

Your program should determine:

您的程序应该确定：

- the total number of actions it should take to reach the desired number of liters, including the first fill of the starting bucket
- which bucket should end up with the desired number of liters - either bucket one or bucket two
- how many liters are left in the other bucket

- 达到所需升数应采取的操作总数，包括起始桶的第一次填充
- 哪个桶最终应具有所需的升数 - 桶一或桶二
- 另一个桶里还剩下多少升

Note: any time a change is made to either or both buckets counts as one (1) action.

注意：任何时候对一个或两个存储桶进行更改都算作一 (1) 次操作。

Example:
Bucket one can hold up to 7 liters, and bucket two can hold up to 11 liters.
Let's say at a given step, bucket one is holding 7 liters and bucket two is holding 8 liters (7,8).
If you empty bucket one and make no change to bucket two, leaving you with 0 liters and 8 liters respectively (0,8), that counts as one action.
Instead, if you had poured from bucket one into bucket two until bucket two was full, resulting in 4 liters in bucket one and 11 liters in bucket two (4,11), that would also only count as one action.

示例：
第一个桶最多可容纳 7 升，第二个桶最多可容纳 11 升。
假设在给定步骤中，第一个桶容纳 7 升，第二个桶容纳 8 升 (7,8)。
如果您清空一号桶且不对二号桶进行任何更改，则分别留下 0 升和 8 升 (0,8)，则算作一次操作。
相反，如果您将第一个桶倒入第二个桶中，直到第二个桶装满，导致第一个桶中倒入 4 升，第二个桶中倒入 11 升 (4,11)，这也只能算作一次操作。

Another Example:
Bucket one can hold 3 liters, and bucket two can hold up to 5 liters.
You are told you must start with bucket one.
So your first action is to fill bucket one.
You choose to empty bucket one for your second action.
For your third action, you may not fill bucket two, because this violates the third rule -- you may not end up in a state after any action where the starting bucket is empty and the other bucket is full.

另一个例子：
第一个桶可容纳 3 升，第二个桶最多可容纳 5 升。
你被告知必须从第一桶开始。
所以你的第一个行动就是装满水桶。
您选择清空第一个桶以进行第二个操作。
对于第三个操作，您可能不会填充第二个桶，因为这违反了第三条规则 - 在执行任何操作后，您可能不会处于起始桶为空而另一个桶已满的状态。

Written with <3 at [Fullstack Academy][fullstack] by Lindsay Levine.

由 Lindsay Levine 在 [Fullstack Academy][fullstack] 与 <3 一起编写。

[fullstack]: https://www.fullstackacademy.com/


[全栈]：https://www.fullstackacademy.com/