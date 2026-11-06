## General

## 一般的

Do not implement the struct `SimpleLinkedList` as a wrapper around a `Vec`. Instead, allocate nodes on the heap.

不要将 struct `SimpleLinkedList` 实现为 `Vec` 的包装器。相反，在堆上分配节点。

This might be implemented as:

这可以实现为：

```
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
```

The `head` field points to the first element (Node) of this linked list.

`head` 字段指向该链表的第一个元素（Node）。

This implementation also requires a struct `Node` with the following fields:

此实现还需要具有以下字段的结构 `Node`：

```
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```

`data` contains the stored data, and `next` points to the following node (if available) or None.

`data` 包含存储的数据，`next` 指向后面的节点（如果可用）或 None。

## Why `Option<Box<Node<T>>>` and not just `Option<Node<T>>`?

## 为什么是 `Option<Box<Node<T>>>` 而不仅仅是 `Option<Node<T>>`？

Try it on your own. You will get the following error.

自己尝试一下。您将收到以下错误。

```
| struct Node<T>
| ^^^^^^^^^^^^^^ recursive type has infinite size
...
|     next: Option<Node<T>>,
|     --------------------- recursive without indirection
```

The problem is that at compile time the size of next must be known.
Since `next` is recursive ("a node has a node has a node..."), the compiler does not know how much memory is to be allocated.
In contrast, [Box](https://doc.rust-lang.org/std/boxed/) is a heap pointer with a defined size.


问题是在编译时必须知道 next 的大小。
由于`next`是递归的（“一个节点有一个节点有一个节点......”），编译器不知道要分配多少内存。
相反，[Box](https://doc.rust-lang.org/std/boxed/) 是一个具有定义大小的堆指针。