# Instructions

You're working on implementing a health-monitoring system. As part of that, you need to keep track of users' health statistics.

You'll start with some stubbed functions in an `impl` block as well as the following struct definition:

```rust
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}
```

Your goal is to implement the stubbed out methods on the `User` `struct` defined in the `impl` block.

For example, the `new` method should return an instance of the `User` struct with the specified name, age, and weight values.

```rust
let mut bob = User::new(String::from("Bob"), 32, 155.2);
// Returns: a User with name "Bob", age 32, and weight 155.2
```

The `weight` method should return the weight of the `User`.

```rust
bob.weight();
// Returns: 155.2
```

The `set_age` method should set the age of the `User`.

```rust
bob.set_age(33);
// Updates Bob's age to 33; happy birthday Bob!
```

Have fun!

中文对照：
本练习实现一个健康监测系统，用于记录用户的健康统计数据。需要实现 `User` 的方法：`new` 创建用户，`weight` 返回体重，`set_age` 更新年龄。
