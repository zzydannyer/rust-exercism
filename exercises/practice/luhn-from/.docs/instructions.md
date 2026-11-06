# Luhn: Using the From Trait

# Luhn：使用 From 特征

Before doing this exercise you should probably do the original Luhn exercise. If you have not completed Luhn, you can get it by running the command:

在进行此练习之前，您可能应该进行原始的 Luhn 练习。如果您还没有完成 Luhn，可以通过运行以下命令来获取：

```shell
exercism download --exercise=luhn --track=rust
```

In the original Luhn exercise you only validated strings, but the Luhn algorithm can be applied to integers as well.

在最初的 Luhn 练习中，您仅验证了字符串，但 Luhn 算法也可以应用于整数。

In this exercise you'll implement the [From trait](https://doc.rust-lang.org/std/convert/trait.From.html) to convert strings, strs and unsigned integers into a Struct that performs the validation.


在本练习中，您将实现 [From trait](https://doc.rust-lang.org/std/convert/trait.From.html) 以将字符串、str 和无符号整数转换为执行验证的结构。