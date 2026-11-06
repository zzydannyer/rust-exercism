# Instructions append

# 追加说明

## Rust Traits for `.to_string()`

## `.to_string()` 的 Rust 特征

You will also need to implement `.to_string()` for the `Clock` struct.
We will be using this to display the Clock's state.
You can either do it via implementing it directly or using the [Display trait][display-trait].

您还需要为 `Clock` 结构实现 `.to_string()`。
我们将使用它来显示时钟的状态。
您可以通过直接实现或使用[显示特征][显示特征]来实现。

If so, try implementing the [Display trait][display-trait] for `Clock` instead.

如果是这样，请尝试为 `Clock` 实现 [显示特征][显示特征]。

Traits allow for a common way to implement functionality for various types.

特征允许采用通用方法来实现各种类型的功能。

For additional learning, consider how you might implement `String::from` for the `Clock` type.
You don't have to actually implement this—it's redundant with `Display`, which is generally the
better choice when the destination type is `String`—but it's useful to have a few type-conversion
traits in your toolkit.

如需进一步学习，请考虑如何为 `Clock` 类型实现 `String::from`。
您不必实际实现这一点 - 它对于 `Display` 来说是多余的，这通常是
当目标类型为 `String` 时更好的选择 - 但进行一些类型转换很有用
你的工具包中的特征。

[display-trait]: https://doc.rust-lang.org/std/fmt/trait.Display.html


[显示特征]：https://doc.rust-lang.org/std/fmt/trait.Display.html