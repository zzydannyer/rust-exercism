# Instructions append

# 追加说明

## Notes on Rust implementation

## Rust 实现注意事项

By using private fields in structs with public `new` functions returning
`Option` or `Result` (as here with `DNA::new` & `RNA::new`), we can guarantee
that the internal representation of `DNA` is correct. Because every valid DNA
string has a valid RNA string, we don't need to return a `Result`/`Option` from
`into_rna`.

通过在结构中使用私有字段并返回公共 `new` 函数
`Option` 或 `Result`（如此处的 `DNA::new` 和 `RNA::new`），我们可以保证
`DNA` 的内部表示是正确的。因为每一个有效的DNA
string 具有有效的 RNA 字符串，我们不需要返回 `Result`/`Option`
ZXQ保护8QXZ。

This explains the type signatures you will see in the tests.

这解释了您将在测试中看到的类型签名。

The return types of both `DNA::new()` and `RNA::new()` are `Result<Self, usize>`,
where the error type `usize` represents the index of the first invalid character
(char index, not utf8).


`DNA::new()` 和 `RNA::new()` 的返回类型均为 `Result<Self, usize>`，
其中错误类型 `usize` 表示第一个无效字符的索引
（字符索引，不是 utf8）。