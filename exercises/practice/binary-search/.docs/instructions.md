# Instructions

# 指示

Your task is to implement a binary search algorithm.

您的任务是实现二分搜索算法。

A binary search algorithm finds an item in a list by repeatedly splitting it in half, only keeping the half which contains the item we're looking for.
It allows us to quickly narrow down the possible locations of our item until we find it, or until we've eliminated all possible locations.

二分搜索算法通过重复地将列表分成两半来查找列表中的项目，只保留包含我们要查找的项目的一半。
它使我们能够快速缩小物品的可能位置范围，直到找到它，或者直到我们消除了所有可能的位置。

~~~~exercism/caution
Binary search only works when a list has been sorted.
~~~~

~~~~锻炼/注意
二分查找仅在列表已排序时才有效。
～～～

The algorithm looks like this:

该算法如下所示：

- Find the middle element of a _sorted_ list and compare it with the item we're looking for.
- If the middle element is our item, then we're done!
- If the middle element is greater than our item, we can eliminate that element and all the elements **after** it.
- If the middle element is less than our item, we can eliminate that element and all the elements **before** it.
- If every element of the list has been eliminated then the item is not in the list.
- Otherwise, repeat the process on the part of the list that has not been eliminated.

- 找到_sorted_列表的中间元素并将其与我们正在查找的项目进行比较。
- 如果中间的元素是我们的项目，那么我们就完成了！
- 如果中间元素大于我们的项目，我们可以消除该元素以及它之后的所有元素。
- 如果中间元素小于我们的项目，我们可以消除该元素以及它之前**的所有元素。
- 如果列表中的每个元素都已被删除，则该项目不在列表中。
- 否则，对列表中未被消除的部分重复该过程。

Here's an example:

这是一个例子：

Let's say we're looking for the number 23 in the following sorted list: `[4, 8, 12, 16, 23, 28, 32]`.

假设我们正在以下排序列表中查找数字 23：`[4, 8, 12, 16, 23, 28, 32]`。

- We start by comparing 23 with the middle element, 16.
- Since 23 is greater than 16, we can eliminate the left half of the list, leaving us with `[23, 28, 32]`.
- We then compare 23 with the new middle element, 28.
- Since 23 is less than 28, we can eliminate the right half of the list: `[23]`.
- We've found our item.


- 我们首先将 23 与中间元素 16 进行比较。
- 由于 23 大于 16，我们可以消除列表的左半部分，留下 `[23, 28, 32]`。
- 然后我们将 23 与新的中间元素 28 进行比较。
- 由于23小于28，我们可以消除列表的右半部分：`[23]`。
- 我们找到了我们的物品。