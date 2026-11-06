# Introduction

[Stacks](https://en.wikipedia.org/wiki/Stack_%28abstract_data_type%29) are a type of collection commonly used in computer science.
They are defined by their two key operations: **push** and **pop**.
**Push** adds an element to the top of the stack.
**Pop** removes and returns the topmost element.

Think of a stack like a stack of plates.
You can either add a plate to the top of the stack or take the topmost plate.
To access something further down, you have to remove all of the plates above it.

Rust's vector implementation, [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), can be used as a stack by using its `push()` and `pop()` methods.
Naturally, `push()` adds an element to the end of the `Vec` and `pop()` removes and returns the last element.
These operation can be very fast (O(1) in [Big O Notation](https://en.wikipedia.org/wiki/Big_O_notation)),
so they are one of the most idiomatic ways to use a `Vec`.

Stacks are useful to hold arbitrary numbers of elements in a specific order.
Because the last element inserted is the first element returned,
stacks are commonly refered to as **LIFO** (Last-In, First-Out).
This inherent ordering can be used for many things,
including tracking state when evaulating **Reverse Polish notation**.

栈是计算机科学中常用的一类集合。
它由两个关键操作定义：**push** 和 **pop**。
**Push** 将元素添加到栈顶，**Pop** 移除并返回栈顶元素。

可以把栈想象成一摞盘子：你可以把盘子放到最上面，也可以取走最上面的盘子。
如果要访问更下面的内容，就必须先移除上面的所有盘子。

Rust 的向量实现 [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) 可以通过 `push()` 和 `pop()` 方法充当栈。
`push()` 会把元素添加到 `Vec` 末尾，而 `pop()` 会移除并返回最后一个元素。

这些操作可能非常快（在[大 O 表示法](https://en.wikipedia.org/wiki/Big_O_notation)中为 O(1)），
因此使用它们是操作 `Vec` 最符合 Rust 惯用方式的方法之一。

栈适合按特定顺序保存任意数量的元素。
由于最后插入的元素会最先返回，栈通常被称为 **LIFO**（后进先出）。
这种固有顺序可用于许多场景，包括在计算**逆波兰表示法**时跟踪状态。
