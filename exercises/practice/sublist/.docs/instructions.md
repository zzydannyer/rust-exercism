# Instructions

# 指示

Given any two lists `A` and `B`, determine if:

给定任意两个列表 `A` 和 `B`，确定是否：

- List `A` is equal to list `B`; or
- List `A` contains list `B` (`A` is a superlist of `B`); or
- List `A` is contained by list `B` (`A` is a sublist of `B`); or
- None of the above is true, thus lists `A` and `B` are unequal

- 列表 `A` 等于列表 `B`；或
- 列表`A`包含列表`B`（`A`是`B`的超级列表）；或
- 列表 `A` 包含在列表 `B` 中（`A` 是 `B` 的子列表）；或
- 以上均不正确，因此列出 `A` 和 `B` 不相等

Specifically, list `A` is equal to list `B` if both lists have the same values in the same order.
List `A` is a superlist of `B` if `A` contains a contiguous sub-sequence of values equal to `B`.
List `A` is a sublist of `B` if `B` contains a contiguous sub-sequence of values equal to `A`.

具体来说，如果两个列表具有相同顺序的相同值，则列表 `A` 等于列表 `B`。
如果 `A` 包含等于 `B` 的连续值子序列，则列表 `A` 是 `B` 的超级列表。
如果 `B` 包含等于 `A` 的连续值子序列，则列表 `A` 是 `B` 的子列表。

Examples:

示例：

- If `A = []` and `B = []` (both lists are empty), then `A` and `B` are equal
- If `A = [1, 2, 3]` and `B = []`, then `A` is a superlist of `B`
- If `A = []` and `B = [1, 2, 3]`, then `A` is a sublist of `B`
- If `A = [1, 2, 3]` and `B = [1, 2, 3, 4, 5]`, then `A` is a sublist of `B`
- If `A = [3, 4, 5]` and `B = [1, 2, 3, 4, 5]`, then `A` is a sublist of `B`
- If `A = [3, 4]` and `B = [1, 2, 3, 4, 5]`, then `A` is a sublist of `B`
- If `A = [1, 2, 3]` and `B = [1, 2, 3]`, then `A` and `B` are equal
- If `A = [1, 2, 3, 4, 5]` and `B = [2, 3, 4]`, then `A` is a superlist of `B`
- If `A = [1, 2, 4]` and `B = [1, 2, 3, 4, 5]`, then `A` and `B` are unequal
- If `A = [1, 2, 3]` and `B = [1, 3, 2]`, then `A` and `B` are unequal


- 如果 `A = []` 和 `B = []` （两个列表均为空），则 `A` 和 `B` 相等
- 如果 `A = [1, 2, 3]` 和 `B = []`，则 `A` 是 `B` 的超级列表
- 如果 `A = []` 和 `B = [1, 2, 3]`，则 `A` 是 `B` 的子列表
- 如果 `A = [1, 2, 3]` 和 `B = [1, 2, 3, 4, 5]`，则 `A` 是 `B` 的子列表
- 如果 `A = [3, 4, 5]` 和 `B = [1, 2, 3, 4, 5]`，则 `A` 是 `B` 的子列表
- 如果 `A = [3, 4]` 和 `B = [1, 2, 3, 4, 5]`，则 `A` 是 `B` 的子列表
- 如果 `A = [1, 2, 3]` 和 `B = [1, 2, 3]`，则 `A` 和 `B` 相等
- 如果 `A = [1, 2, 3, 4, 5]` 和 `B = [2, 3, 4]`，则 `A` 是 `B` 的超级列表
- 如果 `A = [1, 2, 4]` 和 `B = [1, 2, 3, 4, 5]`，则 `A` 和 `B` 不相等
- 如果 `A = [1, 2, 3]` 和 `B = [1, 3, 2]`，则 `A` 和 `B` 不相等