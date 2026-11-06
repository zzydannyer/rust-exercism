# Description

# 描述

Write a doubly linked list using unsafe Rust, including an iterator over the list
and a cursor for efficient mutation.

使用不安全的 Rust 编写双向链表，包括列表上的迭代器
以及用于有效突变的光标。

The doubly linked list is a fundamental data structure in computer science.

双向链表是计算机科学中的基本数据结构。

Each node in a doubly linked list contains data and pointers to the next
and previous node, if they exist.

双向链表中的每个节点都包含数据和指向下一个节点的指针
和前一个节点（如果存在）。

New nodes can be efficiently added at any point in the list, if one already has
a reference to the position. Likewise, all elements
from another list can be inserted at any point in constant time.

如果已有节点，则可以在列表中的任何点有效地添加新节点
对职位的引用。同样，所有元素
可以在恒定时间内的任意点插入另一个列表中的内容。

In Rust, linked lists are very rarely used, but occasionally they trip up
newcomers, when they try implementing one. Often, they find it unexpectedly
difficult to work with the yet unfamiliar borrow checker.

在 Rust 中，链表很少使用，但偶尔也会出错
新来者，当他们尝试实施一项时。很多时候，他们会意外地发现
很难与还不熟悉的借阅检查器一起工作。

## A Note on `unsafe`
Remember, the goal of unsafe Rust is to write safe code in cases where the compiler can't help us
guarantee correctness. It must not be possible for a user to cause memory unsafety of any kind using
only the safe interfaces we expose.

## 关于 `unsafe` 的注释
请记住，不安全 Rust 的目标是在编译器无法帮助我们的情况下编写安全的代码
保证正确性。用户不可能使用以下方式导致任何类型的内存不安全
仅我们公开的安全接口。

Document the safety-critical invariants you need to uphold and comment each unsafe block explaining why it
is safe.

记录您需要维护的安全关键不变量，并对每个不安全块进行注释，解释其原因
是安全的。

Any function where the caller has to maintain safety-critical invariants should be marked unsafe. This includes
private functions.

调用者必须维护安全关键不变量的任何函数都应标记为不安全。这包括
私人职能。

## Step 1

## 步骤 1

Implement the functionality for adding and removing elements (pushing and popping)
at the front and back. This is enough to use the list as a double-ended queue.
Also implement the `len` and `is_empty` functions.

实现添加和删除元素的功能（推送和弹出）
在前面和后面。这足以将列表用作双端队列。
还要实现 `len` 和 `is_empty` 函数。

In the finished implementation, all modifications of the list should be done through the cursor struct
to minimize duplication. The `push_*` and `pop_*` methods on `LinkedList`
are defined in terms of required cursor methods in the module `pre_implemented`. If you wish, you
can skip the `Cursor` struct for now and override the methods, but please revert them at the end.

在完成的实现中，列表的所有修改都应该通过游标结构来完成
以尽量减少重复。 `LinkedList` 上的 `push_*` 和 `pop_*` 方法
根据模块 `pre_implemented` 中所需的游标方法定义。如果你愿意的话，你
现在可以跳过 `Cursor` 结构并覆盖这些方法，但请在最后恢复它们。

## Step 2

## 步骤 2

Implement iteration over the list from front to back with the `Iter` struct.

使用 `Iter` 结构从前到后实现列表迭代。

## Step 3

## 步骤 3

Complete the functionality of the cursor. It should be able to move to any position and insert or remove elements there.

完成光标的功能。它应该能够移动到任何位置并在那里插入或删除元素。

## Step 4

## 步骤 4

Implement the `Drop` trait for your `LinkedList` to clean up resources.

为您的 `LinkedList` 实现 `Drop` 特征以清理资源。

## Step 5 (advanced and optional)

## 步骤 5（高级且可选）

The tests for these last two things are conditionally compiled via the feature flag `advanced`.
Add the key `default = ["advanced"]` to the `Cargo.toml` file under `[features]` to activate them.

最后两件事的测试是通过功能标志 `advanced` 有条件编译的。
将密钥 `default = ["advanced"]` 添加到 `[features]` 下的 `Cargo.toml` 文件中以激活它们。

To allow users of your structure maximum flexibility, make sure that your `LinkedList<T>` is covariant over `T`.
This means, for example, that a `LinkedList<&'static T>` can also be used as a `LinkedList<&'a T>`.
See the [Rustonomicon](https://doc.rust-lang.org/nomicon/subtyping.html#subtyping-and-variance) for an explanation of variance in Rust.

为了让您的结构的用户获得最大的灵活性，请确保您的 `LinkedList<T>` 与 `T` 是协变的。
例如，这意味着 `LinkedList<&'static T>` 也可以用作 `LinkedList<&'a T>`。
请参阅 [Rustonomicon](https://doc.rust-lang.org/nomicon/subtyping.html#subtyping-and-variance) 了解 Rust 中方差的解释。

Make sure that your list is safe to send and share across thread boundaries
and signal this to the type system by implementing `Send` and `Sync` manually.
These traits are usually auto-derived, but aren't implemented here automatically, because of the use of
raw pointers. See the docs for [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) and [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) and the [rustonomicon chapter](https://doc.rust-lang.org/nomicon/send-and-sync.html) on them for details on their significance.


确保您的列表可以安全地跨线程边界发送和共享
并通过手动实现 `Send` 和 `Sync` 向类型系统发出信号。
这些特征通常是自动派生的，但不会在这里自动实现，因为使用了
原始指针。有关其重要性的详细信息，请参阅 [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) 和 [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) 以及 [rustonomicon chapter](https://doc.rust-lang.org/nomicon/send-and-sync.html) 的文档。