## General

## 一般的

* A doubly linked does not have a clear ownership hierarchy, which is why it requires either the use
  of unsafe or abstractions for shared ownership like `Rc`. The latter has some overhead that is unnecessary
  for this case.

* 双向链接没有明确的所有权层次结构，这就是为什么它需要使用
  共享所有权的不安全或抽象，例如 `Rc`。后者有一些不必要的开销
  对于这个案例。

* Refer to the [Rustonomicon](https://doc.rust-lang.org/nomicon/) for details on how to use `unsafe {}` correctly.

*有关如何正确使用`unsafe {}`的详细信息，请参阅[Rustonomicon](https://doc.rust-lang.org/nomicon/)。

* Several functions require similar behaviour in different directions (towards front or back). Try not to duplicate
  shared code paths.


* 多个功能需要在不同方向（向前或向后）类似的行为。尽量不要重复
  共享代码路径。