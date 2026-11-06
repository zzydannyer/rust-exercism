# Instructions

In this exercise you'll be generating semi-structured log messages.

在本练习中，你将生成半结构化日志消息。

## 1. Emit semi-structured messages

## 1. 输出半结构化消息

You'll start with some stubbed functions and the following enum:

```rust
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
```

Your goal is to emit a log message as follows: `"[<LEVEL>]: <MESSAGE>"`.
You'll need to implement functions that correspond with log levels.

你的目标是按以下格式输出日志消息：`"[<LEVEL>]: <MESSAGE>"`。
你需要实现与各日志级别对应的函数。

For example, the below snippet demonstrates an expected output for the `log` function.

```rust
log(LogLevel::Error, "Stack overflow")
// Returns: "[ERROR]: Stack overflow"
```

And for `info`:

对于 `info`，则是：

```rust
info("Timezone changed")
// Returns: "[INFO]: Timezone changed"
```

Have fun!

祝你编程愉快！

## 2. Optional further practice

## 2. 可选的进一步练习

There is a feature-gated test in this suite.
Feature gates disable compilation entirely for certain sections of your program.
They will be covered later.
For now just know that there is a test which is only built and run when you use a special testing invocation:

```sh
cargo test --features add-a-variant
```

This test is meant to show you how to add a variant to your enum.

这个测试旨在展示如何向枚举中添加一个变体。
