## General

## 一般的

[Slices](https://doc.rust-lang.org/book/2018-edition/ch04-03-slices.html) have additionally to
the normal element access via indexing (slice[index]) many useful functions like
[split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at) or [getting
subslices](https://doc.rust-lang.org/std/primitive.slice.html#method.get) (slice[start..end]).

[Slices](https://doc.rust-lang.org/book/2018-edition/ch04-03-slices.html) 另外还有
通过索引（slice[index]）进行普通元素访问许多有用的函数，例如
[split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at) 或 [getting
subslices](https://doc.rust-lang.org/std/primitive.slice.html#method.get)（切片[开始..结束]）。

You can solve this exercise by just using boring old element access via indexing, but maybe the
other provided functions can make your code cleaner and safer.

您可以通过索引使用无聊的旧元素访问来解决此练习，但也许
其他提供的功能可以使您的代码更干净、更安全。

## For Bonus Points

## 奖励积分

- To get your function working with all kind of elements which can be ordered,
  have a look at the [Ord Trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
- To get your function working directly on Vec and Array, you can use the
  [AsRef Trait](https://doc.rust-lang.org/std/convert/trait.AsRef.html)


- 为了让您的函数能够处理所有可以订购的元素，
  看看 [Ord Trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html)。
- 要让您的函数直接在 Vec 和 Array 上工作，您可以使用
  ZXQ保护1QXZ