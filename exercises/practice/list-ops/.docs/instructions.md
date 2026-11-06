# Instructions

# 指示

Implement basic list operations.

实现基本的列表操作。

In functional languages list operations like `length`, `map`, and `reduce` are very common.
Implement a series of basic list operations, without using existing functions.

在函数式语言中，`length`、`map` 和 `reduce` 等列表操作非常常见。
实现一系列基本列表操作，而不使用现有函数。

The precise number and names of the operations to be implemented will be track dependent to avoid conflicts with existing names, but the general operations you will implement include:

要实现的操作的精确数量和名称将取决于轨道，以避免与现有名称冲突，但您将实现的常规操作包括：

- `append` (_given two lists, add all items in the second list to the end of the first list_);
- `concatenate` (_given a series of lists, combine all items in all lists into one flattened list_);
- `filter` (_given a predicate and a list, return the list of all items for which `predicate(item)` is True_);
- `length` (_given a list, return the total number of items within it_);
- `map` (_given a function and a list, return the list of the results of applying `function(item)` on all items_);
- `foldl` (_given a function, a list, and initial accumulator, fold (reduce) each item into the accumulator from the left_);
- `foldr` (_given a function, a list, and an initial accumulator, fold (reduce) each item into the accumulator from the right_);
- `reverse` (_given a list, return a list with all the original items, but in reversed order_).

- `append`（_给定两个列表，将第二个列表中的所有项目添加到第一个列表的末尾_）；
- `concatenate`（_给定一系列列表，将所有列表中的所有项目组合成一个扁平列表_）；
- `filter`（_给定谓词和列表，返回 `predicate(item)` 为 True 的所有项目的列表_）；
- `length`（_给定一个列表，返回其中的项目总数_）；
- `map`（_给定一个函数和一个列表，返回对所有项目应用 `function(item)` 的结果列表_）；
- `foldl`（_给定一个函数、一个列表和初始累加器，将每个项目从左侧折叠（减少）到累加器中_）；
- `foldr`（_给定一个函数、一个列表和一个初始累加器，将每个项目从右侧折叠（减少）到累加器中_）；
- `reverse`（_给定一个列表，返回包含所有原始项目的列表，但顺序相反_）。

Note, the ordering in which arguments are passed to the fold functions (`foldl`, `foldr`) is significant.


请注意，参数传递给折叠函数（`foldl`、`foldr`）的顺序很重要。