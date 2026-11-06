# Instructions append

# 追加说明

## Bonus

## 奖金

Test your function on this string: `uüu` and see what happens.
Try to write a function that properly reverses this string.
Hint: grapheme clusters

在此字符串上测试您的函数：`uüu`，看看会发生什么。
尝试编写一个函数来正确反转该字符串。
提示：字素簇

To get the bonus test to run, remove the ignore flag (`#[ignore]`) from the last test, and execute the tests with:

要运行额外测试，请从上次测试中删除忽略标志 (`#[ignore]`)，然后使用以下命令执行测试：

```bash
cargo test --features grapheme
```

You will need to use external libraries (a `crate` in rust lingo) for the bonus task.
A good place to look for those is [crates.io](https://crates.io/), the official repository of crates.
Please remember that only a limited set of crates is supported by our test runner.
The full list is in [this file](https://github.com/exercism/rust-test-runner/blob/main/local-registry/Cargo.toml) under the section `[dependencies]`.

您将需要使用外部库（Rust 术语中的 `crate`）来完成奖励任务。
寻找这些内容的好地方是 [crates.io](https://crates.io/)，它是板条箱的官方存储库。
请记住，我们的测试运行程序仅支持有限的一组板条箱。
完整列表位于 `[dependencies]` 部分下的 [this file](https://github.com/exercism/rust-test-runner/blob/main/local-registry/Cargo.toml) 中。

[Check the documentation](https://doc.rust-lang.org/cargo/guide/dependencies.html) for instructions on how to use external crates in your projects.


[Check the documentation](https://doc.rust-lang.org/cargo/guide/dependencies.html) 有关如何在项目中使用外部 crate 的说明。