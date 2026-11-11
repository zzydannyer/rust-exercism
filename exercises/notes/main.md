# Notes

### 反转字符串

`&str.chars().rev().collect`

### 字形簇

`&str.graphemes(true).rev().collect()`

### 数组操作

```
 (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n\n")
```
