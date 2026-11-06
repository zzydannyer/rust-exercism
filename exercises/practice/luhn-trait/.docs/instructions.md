# Luhn: Using a Custom Trait

# Luhn：使用自定义特征

Before doing this exercise you should probably do the original Luhn exercise and its successor, "Luhn: Using the From Trait"

在进行此练习之前，您可能应该进行原始 Luhn 练习及其后续练习“Luhn：使用 From Trait”

To get the original Luhn exercise, run

要获得原始的 Luhn 练习，请运行

```shell
exercism download --exercise=luhn --track=rust
```

To get the "Luhn: Using the From Trait" exercise, run

要进行“Luhn：使用 From Trait”练习，请运行

```shell
exercism download --exercise=luhn-from --track=rust
```

In the original Luhn exercise you only validated strings, but the Luhn algorithm can be applied to integers as well.

在最初的 Luhn 练习中，您仅验证了字符串，但 Luhn 算法也可以应用于整数。

In "Luhn: Using the From Trait" you implemented a From trait, which also required you to create a Luhn struct.

在“Luhn：使用 From Trait”中，您实现了 From 特征，这还要求您创建一个 Luhn 结构体。

Instead of creating a Struct just to perform the validation, what if you validated the primitives (i.e, String, u8, etc.) themselves?

如果您不只是创建一个 Struct 来执行验证，而是验证原语（即 String、u8 等）本身会怎样？

In this exercise you'll create and implement a custom [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) that performs the validation.


在本练习中，您将创建并实现执行验证的自定义 [trait](https://doc.rust-lang.org/book/ch10-02-traits.html)。