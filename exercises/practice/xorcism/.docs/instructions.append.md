# Instructions append

# 追加说明

## Lifetime of `munge` return value

## `munge` 返回值的生命周期

Due to the usage of the `impl Trait` feature, lifetime management may be a bit
tricky when implementing the `munge` method. You may find it easier to write
your own `struct` with an `Iterator` implementation and return that concrete
type, at least to get started. Ultimately, it's a good idea to try and implement
the solution using `Iterator` combinators directly.


由于使用了 `impl Trait` 功能，生命周期管理可能会有点困难
实现 `munge` 方法时很棘手。你可能会发现写起来更容易
您自己的 `struct` 与 `Iterator` 实现并返回具体的
键入，至少开始。最终，尝试并实施是一个好主意
直接使用 `Iterator` 组合器的解决方案。