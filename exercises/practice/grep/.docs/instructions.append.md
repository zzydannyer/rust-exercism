# Instructions append

# 追加说明

## Error handling

## 错误处理

This exercise introduces the `anyhow` crate, which makes it easy to handle arbitrary error types.
Its intent is to ensure that when you're writing an application, you don't have to worry about what
particular errors your called function is returning, but to just do the right thing when propagating them.

本练习介绍了 `anyhow` 包，它可以轻松处理任意错误类型。
其目的是确保当您编写应用程序时，您不必担心什么
您被调用的函数返回的特定错误，但在传播它们时只做正确的事情。

N.B.: it is actually somewhat bad form to use `anyhow` when writing a library, as we are here; it's more
explicit and more useful to write your own `Error` enum when writing a library (potentially with the aid of helper
macros such as are provided by the [`thiserror` crate](https://crates.io/crates/thiserror)). However, we are
intentionally and explicitly doing so here to demonstrate the use of this crate.

注意：在编写库时使用 `anyhow` 实际上是一种不好的形式，就像我们在这里一样；更多的是
在编写库时编写自己的 `Error` 枚举更明确且更有用（可能需要帮助程序
例如由 [`thiserror` crate](https://crates.io/crates/thiserror) 提供的宏）。然而，我们是
在这里有意且明确地这样做是为了演示这个板条箱的用法。

To learn more about this crate refer to its [documentation](https://docs.rs/anyhow/1.0.32/anyhow/).

要了解有关此板条箱的更多信息，请参阅其 [documentation](https://docs.rs/anyhow/1.0.32/anyhow/)。

## Additional reading

## 补充阅读

While this exercise asks you to implement only the most basic functions of `grep`,
there is actually a project to fully re-implement `grep` in Rust - [ripgrep](https://github.com/BurntSushi/ripgrep).

虽然本练习要求您仅实现 `grep` 的最基本功能，
实际上有一个项目可以在 Rust 中完全重新实现 `grep` - [ripgrep](https://github.com/BurntSushi/ripgrep)。

If you liked the concept of rewriting the basic util programs in Rust be sure to check the following projects:

如果您喜欢用 Rust 重写基本 util 程序的概念，请务必检查以下项目：

- [fd](https://github.com/sharkdp/fd) - a clone of `find`
- [exa](https://github.com/ogham/exa) - a clone of `ls`
- [bat](https://github.com/sharkdp/bat) - a clone of `cat`
- [coreutils](https://github.com/uutils/coreutils) - a rewrite of the GNU coreutils


- [fd](https://github.com/sharkdp/fd) - `find` 的克隆
- [exa](https://github.com/ogham/exa) - `ls` 的克隆
- [bat](https://github.com/sharkdp/bat) - `cat` 的克隆
- [coreutils](https://github.com/uutils/coreutils) - GNU coreutils 的重写