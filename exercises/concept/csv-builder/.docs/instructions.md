# Instructions

Your company has been hired to generate CSV records from user input.

We need a builder for a CSV record per the standard defined in [RFC 4180](https://tools.ietf.org/html/rfc4180).

While a formal specification exists, your company only needs to implement the following functionality:
- create a new record builder
- add an individual field to the record
- serialize the record with basic escaping

Basic escaping means:

- Fields containing line breaks, double-quotes, and commas should be enclosed in double-quotes.

- If double-quotes are used to enclose fields, then a double-quote appearing inside a field must be escaped by preceding it with another double-quote.


## 1. Create a new builder

You'll need to implement the `new` method to create a new builder.

```rust
let mut builder = CsvRecordBuilder::new();
```

## 2. Add individual fields to the record

You'll need to implement the `add` method to append fields to your record builder.
`add` takes immutable string slices, `&str`, as the argument.

```rust
builder.add("ant");
builder.add("ba\"t");
builder.add("cat");
```


## 3. Serialize the record with basic escaping

You'll need to implement the `build` method.
`build` consumes the builder and returns a CSV record.
```rust
let record = builder.build();
// Note that from now on we cannot use `builder`
assert_eq!(&record, r#"ant,"ba""t",cat"#);
```

中文对照：
本练习根据 RFC 4180 从用户输入生成 CSV 记录。需要创建记录构建器、添加字段，并使用基本转义序列化记录。包含换行、双引号或逗号的字段必须用双引号包围，内部双引号必须再增加一个双引号进行转义。
