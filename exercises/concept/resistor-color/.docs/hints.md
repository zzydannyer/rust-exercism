# Hints

## General

- Use the `enum-iterator` crate to be able to iterate over enums. Check out [docs](https://docs.rs/enum-iterator/1.2.0/enum_iterator/) to find out how.

- Use the `int-enum` crate to be able to convert from int to enum. See the [docs](https://docs.rs/int-enum/0.5.0/int_enum/) for details.

- The `Debug` trait is there because you'll likely need it.

- You'll need to extend the list of `derive`d traits on your colors enum.

中文对照：
使用 `enum-iterator` crate 遍历枚举，使用 `int-enum` crate 将整数转换为枚举。`Debug` trait 也会派上用场，并且需要扩展颜色枚举派生的 trait 列表。
