# Instructions append

# 追加说明

## Abstraction over Networks and Files

## 网络和文件的抽象

Network and file operations are implemented in terms of the [`io::Read`][read] and [`io::Write`][write] traits. It will therefore be necessary to implement those traits for your types.

网络和文件操作是根据 [`io::Read`][read] 和 [`io::Write`][write] 特征实现的。因此，有必要为您的类型实现这些特征。

[read]: https://doc.rust-lang.org/std/io/trait.Read.html
[write]: https://doc.rust-lang.org/std/io/trait.Write.html


[阅读]：https://doc.rust-lang.org/std/io/trait.Read.html
[写入]：https://doc.rust-lang.org/std/io/trait.Write.html