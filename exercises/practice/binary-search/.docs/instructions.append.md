# Instructions append

# 追加说明

## Restrictions

## 限制

Rust provides in its standard library already a
[binary search function](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search).
For this exercise you should not use this function but just other basic tools instead.

Rust 在其标准库中已经提供了
ZXQ保护0QXZ。
对于本练习，您不应使用此功能，而应使用其他基本工具。

## For bonus points

## 奖励积分

Did you get the tests passing and the code clean? If you want to, there
are some additional things you could try.

您是否通过了测试并且代码干净了？如果你愿意的话，那里有
您可以尝试一些其他的事情。

- Currently your find function will probably only work for slices of numbers,
  but the Rust type system is flexible enough to create a find function which
  works on all slices which contains elements which can be ordered.
- Additionally this find function can work not only on slices, but at the
  same time also on a Vec or an Array.

- 目前您的查找功能可能仅适用于数字切片，
  但 Rust 类型系统足够灵活，可以创建一个查找函数
  适用于包含可排序元素的所有切片。
- 此外，此查找功能不仅可以在切片上工作，还可以在
  同时也在 Vec 或数组上。

To run the bonus tests, remove the `#[ignore]` flag and execute the tests with
the `generic` feature, like this:

要运行奖励测试，请删除 `#[ignore]` 标志并使用以下命令执行测试
`generic` 功能，如下所示：

```bash
$ cargo test --features generic
```

Then please share your thoughts in a comment on the submission. Did this
experiment make the code better? Worse? Did you learn anything from it?


然后请在提交的评论中分享您的想法。做了这个
实验让代码变得更好？更糟糕的是？你从中学到什么了吗？