# Instructions

# 指示

Create an implementation of the rotational cipher, also sometimes called the Caesar cipher.

创建旋转密码的实现，有时也称为凯撒密码。

The Caesar cipher is a simple shift cipher that relies on transposing all the letters in the alphabet using an integer key between `0` and `26`.
Using a key of `0` or `26` will always yield the same output due to modular arithmetic.
The letter is shifted for as many values as the value of the key.

凯撒密码是一种简单的移位密码，它依赖于使用 `0` 和 `26` 之间的整数密钥调换字母表中的所有字母。
由于模运算，使用 `0` 或 `26` 密钥将始终产生相同的输出。
字母会移动与键值一样多的值。

The general notation for rotational ciphers is `ROT + <key>`.
The most commonly used rotational cipher is `ROT13`.

旋转密码的一般表示法是 `ROT + <key>`。
最常用的旋转密码是 `ROT13`。

A `ROT13` on the Latin alphabet would be as follows:

拉丁字母上的 `ROT13` 如下：

```text
Plain:  abcdefghijklmnopqrstuvwxyz
Cipher: nopqrstuvwxyzabcdefghijklm
```

It is stronger than the Atbash cipher because it has 27 possible keys, and 25 usable keys.

它比 Atbash 密码更强，因为它有 27 个可能的密钥和 25 个可用密钥。

Ciphertext is written out in the same formatting as the input including spaces and punctuation.

密文以与输入相同的格式写出，包括空格和标点符号。

## Examples

## 示例

- ROT5 `omg` gives `trl`
- ROT0 `c` gives `c`
- ROT26 `Cool` gives `Cool`
- ROT13 `The quick brown fox jumps over the lazy dog.` gives `Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`
- ROT13 `Gur dhvpx oebja sbk whzcf bire gur ynml qbt.` gives `The quick brown fox jumps over the lazy dog.`


- ROT5 `omg` 给出 `trl`
- ROT0 `c` 给出 `c`
- ROT26 `Cool` 给出 `Cool`
- ROT13 `The quick brown fox jumps over the lazy dog.` 给出 `Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`
- ROT13 `Gur dhvpx oebja sbk whzcf bire gur ynml qbt.` 给出 `The quick brown fox jumps over the lazy dog.`