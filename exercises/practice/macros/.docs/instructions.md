# Description

# 描述

Macros are a powerful part of a Rust programmer's toolkit, and [macros by example](https://doc.rust-lang.org/reference/macros-by-example.html) are a relatively simple way to access this power. Let's write one!

宏是 Rust 程序员工具包的强大组成部分，而 [macros by example](https://doc.rust-lang.org/reference/macros-by-example.html) 是访问此功能的相对简单的方法。我们来写一个吧！

## Context

## 语境

What is a macro? [Wikipedia](https://en.wikipedia.org/wiki/Macro_(computer_science)) describes it thus:

什么是宏？ [Wikipedia](https://en.wikipedia.org/wiki/Macro_(computer_science)）是这样描述的：

> A macro (short for "macroinstruction", from Greek μακρός 'long') in computer science is a rule or pattern that specifies how a certain input sequence (often a sequence of characters) should be mapped to a replacement output sequence (also often a sequence of characters) according to a defined procedure. The mapping process that instantiates (transforms) a macro use into a specific sequence is known as macro expansion.

> 计算机科学中的宏（“宏指令”的缩写，来自希腊语 μακρός“长”）是一种规则或模式，指定如何根据定义的过程将某个输入序列（通常是字符序列）映射到替换输出序列（通常也是字符序列）。将宏使用实例化（转换）为特定序列的映射过程称为宏扩展。

Illuminating! But to be more concrete, macros are a special syntax which allows you to generate code at compile time. Macros can be used for compile-time calculation, but more often they're just another way to abstract your code. For example, you've probably already used `println!()` and `vec![]`. These each take an arbitrary number of arguments, so you can't express them as simple functions. On the other hand, they always expand to some amount of absolutely standard Rust code. If you're interested, you can use the [cargo expand](https://github.com/dtolnay/cargo-expand) subcommand to view the results of macro expansion in your code.

照亮！但更具体地说，宏是一种特殊的语法，它允许您在编译时生成代码。宏可用于编译时计算，但更常见的是它们只是抽象代码的另一种方式。例如，您可能已经使用过 `println!()` 和 `vec![]`。它们各自接受任意数量的参数，因此您不能将它们表示为简单的函数。另一方面，它们总是扩展到一定数量的绝对标准的 Rust 代码。如果您有兴趣，可以使用 [cargo expand](https://github.com/dtolnay/cargo-expand) 子命令来查看代码中宏扩展的结果。

For further information about macros in Rust, The Rust Book has a [good chapter](https://doc.rust-lang.org/book/ch19-06-macros.html) on them.

有关 Rust 中宏的更多信息，The Rust Book 上有 [good chapter](https://doc.rust-lang.org/book/ch19-06-macros.html)。

## Problem Statement

## 问题陈述

You can produce a `Vec` of arbitrary length inline by using the `vec![]` macro. However, Rust doesn't come with a way to produce a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) inline. Rectify this by writing a `hashmap!()` macro.

您可以使用 `vec![]` 宏生成任意长度的内联 `Vec`。然而，Rust 没有提供生成 [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) 内联的方法。通过编写 `hashmap!()` 宏来纠正此问题。

For example, a user of your library might write `hashmap!('a' => 3, 'b' => 11, 'z' => 32)`. This should expand to the following code:

例如，您的库的用户可能会编写 `hashmap!('a' => 3, 'b' => 11, 'z' => 32)`。这应该扩展到以下代码：

```rust
{
   let mut hm = HashMap::new();
   hm.insert('a', 3);
   hm.insert('b', 11);
   hm.insert('z', 32);
   hm
}
```

Note that the [`maplit` crate](https://crates.io/crates/maplit) provides a macro which perfectly solves this exercise. Please implement your own solution instead of using this crate; please make an attempt on your own before viewing its source.


请注意，[`maplit` crate](https://crates.io/crates/maplit) 提供了一个可以完美解决此练习的宏。请实现您自己的解决方案，而不是使用这个箱子；请在查看其源代码之前自行尝试。