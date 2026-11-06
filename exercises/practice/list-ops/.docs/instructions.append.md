# Instructions append

# 追加说明

## Implementing your own list operations

## 实现你自己的列表操作

Rust has a rich iterator system, using them to solve this exercise is trivial.
Please attempt to solve the exercise without reaching for iterator methods like
`map()`, `flatten()`, `filter()`, `count()`, `fold()`, `chain()`, `rev()` and similar.
The `next()` and `next_back()` methods do not count.

Rust 拥有丰富的迭代器系统，使用它们来解决这个练习是微不足道的。
请尝试解决这个练习，而不需要使用迭代器方法，例如
`map()`、`flatten()`、`filter()`、`count()`、`fold()`、`chain()`、`rev()` 及类似产品。
`next()` 和 `next_back()` 方法不计算在内。

## Avoiding allocations

## 避免分配

The exercise is solvable without introducing any additional memory allocations in the solution.
See if you can create your own iterator type instead of creating an iterator from a collection.


该练习无需在解决方案中引入任何额外的内存分配即可解决。
看看是否可以创建自己的迭代器类型，而不是从集合创建迭代器。