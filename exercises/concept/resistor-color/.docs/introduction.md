# Introduction

Most programs have dependencies on some libraries. Managing those by hand can be a pain. Luckily, the Rust ecosystem comes standard with `cargo`! `cargo` can manage dependencies for a project.

You will often see external packages being referred to as "crates" in Rust. A crate is a compilation unit in Rust. A crate can be compiled into a binary or into a library.

Most of the time, adding an external dependency is as simple as adding a line to your `Cargo.toml` file.

In this exercise, [`enum-iterator`](https://docs.rs/enum-iterator/1.2.0/enum_iterator/) and [`int-enum`](https://docs.rs/int-enum/0.5.0/int_enum/) dependencies were added for you.

中文对照：
大多数程序依赖某些库，手动管理依赖可能很麻烦。Rust 生态系统自带 `cargo`，可用于管理项目依赖。外部包通常称为 crate；crate 是 Rust 的编译单元，可以编译为二进制文件或库。本练习已经添加了 `enum-iterator` 和 `int-enum` 依赖。
