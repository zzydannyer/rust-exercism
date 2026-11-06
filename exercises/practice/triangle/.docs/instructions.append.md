# Instructions append

# 追加说明

- [Result](https://doc.rust-lang.org/std/result/index.html)

- ZXQ保护0QXZ

Implementation of this can take many forms. Here are some topics that may help you, depending on the approach you take.

其实施可以采取多种形式。以下是一些可能对您有帮助的主题，具体取决于您采取的方法。

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

- ZXQ保护0QXZ
- ZXQ保护1QXZ
- ZXQ保护2QXZ

Or maybe you will come up with an approach that uses none of those!

或者也许您会想出一种不使用这些的方法！

## Non-integer lengths

## 非整数长度

The base exercise tests identification of triangles whose sides are all
integers. However, some triangles cannot be represented by pure integers. A simple example is a triangle with a 90 degree angle between two equal sides of length 1. Its third side has the length square root of 2, which is an irrational number (meaning it cannot be written as an integer or a fraction).

基本练习测试边长均为 的三角形的识别
整数。然而，有些三角形不能用纯整数表示。一个简单的例子是一个长度为 1 的两条等边之间形成 90 度角的三角形。它的第三条边的长度平方根为 2，这是一个无理数（意味着它不能写成整数或分数）。

It would be tedious to rewrite the analysis functions to handle both integer and floating-point cases, and particularly tedious to do so for all potential integer and floating point types: given signed and unsigned variants of bitwidths 8, 16, 32, 64, and 128, that would be 10 reimplementations of fundamentally the same code even before considering floats!

重写分析函数来处理整数和浮点情况会很乏味，尤其是对于所有潜在的整数和浮点类型这样做：给定位宽 8、16、32、64 和 128 的有符号和无符号变体，即使在考虑浮点数之前，这也将是基本相同代码的 10 次重新实现！

There's a better way: [generics](https://doc.rust-lang.org/stable/book/ch10-00-generics.html). By rewriting your Triangle as a `Triangle<T>`, you can write your code once, and hand off the work of generating all those specializations to the compiler. Note that in order to use mathematical operations, you'll need to constrain your generic type to types which support those operations using traits.

有一个更好的方法：[generics](https://doc.rust-lang.org/stable/book/ch10-00-generics.html)。通过将三角形重写为 `Triangle<T>`，您可以编写一次代码，并将生成所有这些专业化的工作交给编译器。请注意，为了使用数学运算，您需要将泛型类型限制为支持使用特征的这些运算的类型。

There are some bonus tests you can run which test your implementation on floating-point numbers. To enable them, run your tests with the `generic` feature flag, like this:

您可以运行一些额外的测试来测试浮点数的实现。要启用它们，请使用 `generic` 功能标志运行测试，如下所示：

```bash
cargo test --features generic
```
