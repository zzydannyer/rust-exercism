## General

## 一般的

- Ranking a list of poker hands can be considered a sorting problem.
- Rust provides the [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) method for `Vec<T> where T: Ord`.
- You might consider implementing a type representing a poker hand which implements [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html).

- 对一组扑克手牌进行排名，可以看作一个排序问题。
- Rust 为 `Vec<T> where T: Ord` 提供了 [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) 方法。
- 您可以考虑实现一个表示扑克手牌的类型，并为其实现 [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)。
