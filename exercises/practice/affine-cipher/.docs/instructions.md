# Instructions

# 指示

Create an implementation of the affine cipher, an ancient encryption system created in the Middle East.

创建仿射密码的实现，仿射密码是中东创建的一种古老的加密系统。

The affine cipher is a type of monoalphabetic substitution cipher.
Each character is mapped to its numeric equivalent, encrypted with a mathematical function and then converted to the letter relating to its new numeric value.
Although all monoalphabetic ciphers are weak, the affine cipher is much stronger than the Atbash cipher, because it has many more keys.

仿射密码是一种单字母替换密码。
每个字符都映射到其对应的数字，使用数学函数加密，然后转换为与其新数字值相关的字母。
尽管所有单字母密码都很弱，但仿射密码比 Atbash 密码强得多，因为它有更多的密钥。

[//]: # " monoalphabetic as spelled by Merriam-Webster, compare to polyalphabetic "

[//]: # “按韦氏词典拼写的单字母，与多字母进行比较”

## Encryption

## 加密

The encryption function is:

加密函数为：

```text
E(x) = (ai + b) mod m
```

Where:

在哪里：

- `i` is the letter's index from `0` to the length of the alphabet - 1.
- `m` is the length of the alphabet.
  For the Latin alphabet `m` is `26`.
- `a` and `b` are integers which make up the encryption key.

- `i` 是从 `0` 到字母表长度 - 1 的字母索引。
- `m` 是字母表的长度。
  对于拉丁字母 `m` 来说是 `26`。
- `a` 和 `b` 是组成加密密钥的整数。

Values `a` and `m` must be _coprime_ (or, _relatively prime_) for automatic decryption to succeed, i.e., they have number `1` as their only common factor (more information can be found in the [Wikipedia article about coprime integers][coprime-integers]).
In case `a` is not coprime to `m`, your program should indicate that this is an error.
Otherwise it should encrypt or decrypt with the provided key.

值 `a` 和 `m` 必须是 _coprime_（或_相对素数_）才能成功自动解密，即，它们将数字 `1` 作为唯一的公因子（更多信息可以在 [关于 coprime 整数的维基百科文章][coprime-integers] 中找到）。
如果 `a` 与 `m` 不互质，则您的程序应指示这是一个错误。
否则，它应该使用提供的密钥进行加密或解密。

For the purpose of this exercise, digits are valid input but they are not encrypted.
Spaces and punctuation characters are excluded.
Ciphertext is written out in groups of fixed length separated by space, the traditional group size being `5` letters.
This is to make it harder to guess encrypted text based on word boundaries.

出于本练习的目的，数字是有效输入，但未加密。
不包括空格和标点符号。
密文以空格分隔的固定长度组写出，传统的组大小为 `5` 字母。
这是为了让根据单词边界猜测加密文本变得更加困难。

## Decryption

## 解密

The decryption function is:

解密函数为：

```text
D(y) = (a^-1)(y - b) mod m
```

Where:

在哪里：

- `y` is the numeric value of an encrypted letter, i.e., `y = E(x)`
- it is important to note that `a^-1` is the modular multiplicative inverse (MMI) of `a mod m`
- the modular multiplicative inverse only exists if `a` and `m` are coprime.

- `y` 是加密字母的数值，即 `y = E(x)`
- 值得注意的是，`a^-1` 是 `a mod m` 的模乘逆 (MMI)
- 仅当 `a` 和 `m` 互质时才存在模乘逆。

The MMI of `a` is `x` such that the remainder after dividing `ax` by `m` is `1`:

`a` 的 MMI 为 `x`，使得 `ax` 除以 `m` 后的余数为 `1`：

```text
ax mod m = 1
```

More information regarding how to find a Modular Multiplicative Inverse and what it means can be found in the [related Wikipedia article][mmi].

有关如何查找模乘逆及其含义的更多信息，请参阅[相关维基百科文章][mmi]。

## General Examples

## 一般示例

- Encrypting `"test"` gives `"ybty"` with the key `a = 5`, `b = 7`
- Decrypting `"ybty"` gives `"test"` with the key `a = 5`, `b = 7`
- Decrypting `"ybty"` gives `"lqul"` with the wrong key `a = 11`, `b = 7`
- Decrypting `"kqlfd jzvgy tpaet icdhm rtwly kqlon ubstx"` gives `"thequickbrownfoxjumpsoverthelazydog"` with the key `a = 19`, `b = 13`
- Encrypting `"test"` with the key `a = 18`, `b = 13` is an error because `18` and `26` are not coprime

- 加密 `"test"` 为 `"ybty"` 提供密钥 `a = 5`、`b = 7`
- 解密 `"ybty"` 给出 `"test"` 密钥 `a = 5`、`b = 7`
- 解密 `"ybty"` 会为 `"lqul"` 提供错误的密钥 `a = 11`、`b = 7`
- 解密`"kqlfd jzvgy tpaet icdhm rtwly kqlon ubstx"`会得到`"thequickbrownfoxjumpsoverthelazydog"`的密钥`a = 19`，`b = 13`
- 使用密钥 `a = 18`、`b = 13` 加密 `"test"` 是错误的，因为 `18` 和 `26` 不是互质的

## Example of finding a Modular Multiplicative Inverse (MMI)

## 求模乘逆 (MMI) 的示例

Finding MMI for `a = 15`:

查找 `a = 15` 的 MMI：

- `(15 * x) mod 26 = 1`
- `(15 * 7) mod 26 = 1`, ie. `105 mod 26 = 1`
- `7` is the MMI of `15 mod 26`

- ZXQ保护0QXZ
- `(15 * 7) mod 26 = 1`，即。 ZXQ保护2QXZ
- `7`是`15 mod 26`的MMI

[mmi]: https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
[coprime-integers]: https://en.wikipedia.org/wiki/Coprime_integers


[毫米米]：https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
[互质整数]：https://en.wikipedia.org/wiki/Coprime_integers