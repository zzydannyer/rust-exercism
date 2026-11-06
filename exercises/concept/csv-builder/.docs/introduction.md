`String` and `&str` are two fundamental ways of working with textual data in Rust.

`String` is an owned, vector of `u8` bytes and mutable.
`&str` is a borrowed reference to a String slice and thus immutable.

You can also create and modify `String`'s using one of their many methods, such as `push`.
```rust
let mut a_string = String::from("hello");
a_string.push('!');
```

You can also create a `String` by converting a `&str`:
```rust
let a_string = "hello".to_string();
```

中文对照：
`String` 和 `&str` 是 Rust 中处理文本的两种基本方式。`String` 是拥有所有权且可变的 `u8` 字节向量；`&str` 是字符串切片的借用引用，因此不可变。可以使用 `push` 等方法创建和修改 `String`，也可以把 `&str` 转换为 `String`。
