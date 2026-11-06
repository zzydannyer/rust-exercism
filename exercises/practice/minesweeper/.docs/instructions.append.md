# Instructions append

# 追加说明

## Performance Hint

## 性能提示

All the inputs and outputs are in ASCII.
Rust `String`s and `&str` are utf8, so while one might expect `"Hello".chars()` to be simple, it actually has to check each char to see if it's 1, 2, 3 or 4 `u8`s long.
If we know a `&str` is ASCII then we can call `.as_bytes()` and refer to the underlying data as a `&[u8]` (byte slice).
Iterating over a slice of ASCII bytes is much quicker as there are no codepoints involved - every ASCII byte is one `u8` long.

所有输入和输出均采用 ASCII 格式。
Rust `String`s 和 `&str` 是 utf8，因此虽然人们可能认为 `"Hello".chars()` 很简单，但实际上它必须检查每个字符以查看其长度是否为 1、2、3 或 4 个 `u8`。
如果我们知道 `&str` 是 ASCII，那么我们可以调用 `.as_bytes()` 并将底层数据引用为 `&[u8]`（字节切片）。
迭代 ASCII 字节切片要快得多，因为不涉及代码点 - 每个 ASCII 字节都是一个 `u8` 长。

Can you complete the challenge without cloning the input?


您能在不克隆输入的情况下完成挑战吗？