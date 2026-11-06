# Description

# 描述

Write a streaming adaptor which contains a reference to a key, and bitwise-XORs
it with arbitrary data.

编写一个流适配器，其中包含对密钥的引用和按位异或
它与任意数据。

XOR is a fundamental binary operation: for each bit in the inputs, set the
corresponding bit of the output to `1` if the input bits are different. If both
inputs are `1` or both are `0`, then the corresponding output bit is `0`.

XOR 是一种基本的二进制运算：对于输入中的每一位，设置
如果输入位不同，则输出到 `1` 的相应位。如果两者都
输入为 `1` 或两者均为 `0`，则对应的输出位为 `0`。

When XORing a document with a key, the key is repeated as many times as
necessary, producing an output document equal in length to the input document.

当对文档与密钥进行异或时，该密钥会重复多次
必要时，生成一个与输入文档长度相等的输出文档。

XORing a document with a key has been used for cryptography as recently as the
early 1900s. While this is thoroughly obsolete as a method for hiding data, it
can be surprisingly useful for generating noisy random-seeming data without
needing the complication of true randomness. It is still used occasionally in
modern cryptography for certain ciphers: the cipher itself is just a mechanism
for generating a very random, infinitely long key, which is XOR'd with the
document.

使用密钥对文档进行异或运算最近已被用于密码学
1900 年代初期。虽然这种隐藏数据的方法已经完全过时了，但它
对于生成嘈杂的随机数据非常有用，而无需
需要真正随机性的复杂性。它仍然偶尔被使用在
某些密码的现代密码学：密码本身只是一种机制
用于生成一个非常随机、无限长的密钥，与
文档。

One interesting property of XOR encryption is that it is symmetrical: XORing any
number with itself produces `0`, and XORing any number with `0` returns the
input number unchanged. Therefore, to decrypt a document which has been
XOR-encrypted, XOR-encrypt it again using the same key.

XOR 加密的一个有趣的特性是它是对称的：对任意值进行异或
数字与其自身产生 `0`，并且将任何数字与 `0` 进行异或返回
输入号码不变。因此，要解密已被
异或加密，使用相同的密钥再次异或加密。

## Nonallocation

## 非分配

It is not practical to write a test which ensures that your struct holds a
reference to the key instead of copying it. Likewise, it is not practical to
prove with a test that neither `munge` nor `munge_in_place`, nor any of their
helper functions, allocate on the heap. Nevertheless, you should attempt to
write your solution in this way.

编写一个测试来确保您的结构持有一个是不切实际的
引用密钥而不是复制它。同样，这是不切实际的
通过测试证明 `munge` 和 `munge_in_place` 以及它们的任何一个
辅助函数，在堆上分配。尽管如此，您应该尝试
用这种方式写出你的解决方案。

## Implementation

## 执行

You will need to write a `struct Xorcism` which holds a reference to a key. That
struct must provide two methods: `munge_in_place` and `munge`. The former
adjusts a byte buffer in-place. The latter is an iterator adaptor: it accepts an
arbitrary iterator of data, and returns a new iterator of data.

您将需要编写一个 `struct Xorcism` 来保存对密钥的引用。那
struct 必须提供两个方法：`munge_in_place` 和 `munge`。前者
就地调整字节缓冲区。后者是一个迭代器适配器：它接受
数据的任意迭代器，并返回数据的新迭代器。

This exercise's stub signatures are largely correct in syntax, but they do not
compile: a large part of the point of this exercise is for you to get familiar
with using lifetimes and generics, so you will need to fill them in on your own.
Another goal of this exercise is for you to figure out an appropriate
factorization which enables you to implement both of those methods with minimal
duplication of effort. Don't be afraid to introduce additional helpers!

本练习的存根签名在语法上基本上是正确的，但它们并不正确
编译：这个练习的很大一部分目的是让你熟悉
使用生命周期和泛型，因此您需要自己填写它们。
本练习的另一个目标是让您找出合适的
因式分解使您能够以最少的成本实现这两种方法
重复努力。不要害怕引入额外的帮手！

## Useful Traits

## 有用的特征

These traits will be useful:

这些特征将很有用：

- [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
- [`Borrow`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)
- [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
- [`Sized`](https://doc.rust-lang.org/std/marker/trait.Sized.html)

- ZXQ保护0QXZ
- ZXQ保护1QXZ
- ZXQ保护2QXZ
- [`Sized`](https://doc.rust-lang.org/std/marker/trait.Sized.html)

## Bonus Tests

## 奖励测试

This exercise contains bonus tests, behind the `io` feature flag. To enable
them, run

此练习包含 `io` 功能标志后面的奖励测试。启用
他们，跑

```sh
cargo test --features io
```

For these tests, you will need to implement a method `reader` with the signature

对于这些测试，您需要实现带有签名的方法 `reader`

```rust
fn reader(self, impl Read) -> impl Read
```

and a method `writer` with the signature

以及带有签名的方法 `writer`

```rust
fn writer(self, impl Write) -> impl Write
```

These functions each convert the `Xorcism` struct into a stream adaptor in the
appropriate direction. They use these traits:

这些函数各自将 `Xorcism` 结构转换为流适配器
适当的方向。他们利用这些特征：

- [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html)
- [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html)


- ZXQ保护0QXZ
- ZXQ保护1QXZ