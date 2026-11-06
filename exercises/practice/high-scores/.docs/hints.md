## General

## 一般的

Consider retaining a reference to `scores` in the struct - copying is not
necessary. You will require some lifetime annotations, though.


考虑在结构中保留对 `scores` 的引用 - 复制不是
必要的。不过，您将需要一些终生注释。