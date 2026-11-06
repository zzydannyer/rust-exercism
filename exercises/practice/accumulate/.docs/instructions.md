# Instructions

# 指示

Implement the `accumulate` operation, which, given a collection and an operation to perform on each element of the collection, returns a new collection containing the result of applying that operation to each element of the input collection.

实现 `accumulate` 操作，在给定集合和对集合的每个元素执行的操作的情况下，该操作返回一个新集合，其中包含将该操作应用于输入集合的每个元素的结果。

Given the collection of numbers:

给定数字的集合：

- 1, 2, 3, 4, 5

And the operation:

以及操作：

- square a number (`x => x * x`)

- 一个数字的平方 (`x => x * x`)

Your code should be able to produce the collection of squares:

您的代码应该能够生成正方形集合：

- 1, 4, 9, 16, 25

Check out the test suite to see the expected function signature.

查看测试套件以查看预期的函数签名。

## Restrictions

## 限制

Keep your hands off that collect/map/fmap/whatchamacallit functionality provided by your standard library!
Solve this one yourself using other basic tools instead.


别再用标准库提供的collect/map/fmap/whatchamacallit 功能了！
使用其他基本工具自己解决这个问题。