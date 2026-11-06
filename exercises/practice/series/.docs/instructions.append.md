# Instructions append

# 追加说明

## Handling edge cases

## 处理边缘情况

Different languages on Exercism have different expectations about what the result should be if the length of the substrings is zero.
On the Rust track, we don't have a test for that case, so you are free to do what you feel is most appropriate.

如果子字符串的长度为零，不同的 Exercism 语言对于结果应该是什么有不同的期望。
在 Rust 赛道上，我们没有针对这种情况的测试，因此您可以自由地做您认为最合适的事情。

Consider the advantages and disadvantages of the following possibilities:
- Crash the program with `panic!`.
- Return a `Result::Err`. (not possible here, because the function signature is given)
- Return an empty vector.
- Return a vector containing as many empty strings as the length of the string "digits" **plus one**. (this has some nice mathematical properties!)


考虑以下可能性的优点和缺点：
- 使用 `panic!` 使程序崩溃。
- 返回 `Result::Err`。 （这里不可能，因为给出了函数签名）
- 返回一个空向量。
- 返回一个向量，其中包含与字符串“digits”**加一**的长度一样多的空字符串。 （这有一些很好的数学特性！）