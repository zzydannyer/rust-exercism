## General

## 一般的

- Instead of implementing arbitrary-precision arithmetic from scratch, consider building your type on top of the [num_bigint](https://crates.io/crates/num-bigint) crate.
- You might be able to [derive](https://doc.rust-lang.org/book/2018-edition/appendix-03-derivable-traits.html) some of the required traits.
- `Decimal` is assumed to be a signed type. You do not have to create a separate unsigned type, though you may do so as an implementation detail if you so choose.


- 考虑在 [num_bigint](https://crates.io/crates/num-bigint) 板条箱之上构建您的类型，而不是从头开始实现任意精度算术。
- 您也许能够 [derive](https://doc.rust-lang.org/book/2018-edition/appendix-03-derivable-traits.html) 一些必需的特征。
- `Decimal` 被假定为有符号类型。您不必创建单独的无符号类型，但如果您愿意，可以将其作为实现细节来创建。