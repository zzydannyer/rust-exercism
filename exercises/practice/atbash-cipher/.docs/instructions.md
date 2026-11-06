# Instructions

# 指示

Create an implementation of the Atbash cipher, an ancient encryption system created in the Middle East.

创建 Atbash 密码的实现，Atbash 密码是中东创建的一种古老的加密系统。

The Atbash cipher is a simple substitution cipher that relies on transposing all the letters in the alphabet such that the resulting alphabet is backwards.
The first letter is replaced with the last letter, the second with the second-last, and so on.

Atbash 密码是一种简单的替换密码，它依赖于调换字母表中的所有字母，从而得到的字母表是向后的。
第一个字母替换为最后一个字母，第二个字母替换为倒数第二个字母，依此类推。

An Atbash cipher for the Latin alphabet would be as follows:

拉丁字母的 Atbash 密码如下：

```text
Plain:  abcdefghijklmnopqrstuvwxyz
Cipher: zyxwvutsrqponmlkjihgfedcba
```

It is a very weak cipher because it only has one possible key, and it is a simple mono-alphabetic substitution cipher.
However, this may not have been an issue in the cipher's time.

它是一种非常弱的密码，因为它只有一个可能的密钥，并且它是一种简单的单字母替换密码。
然而，这在密码时代可能不是问题。

Ciphertext is written out in groups of fixed length, the traditional group size being 5 letters, leaving numbers unchanged, and punctuation is excluded.
This is to make it harder to guess things based on word boundaries.
All text will be encoded as lowercase letters.

密文以固定长度的组写出，传统的组大小为 5 个字母，数字不变，并且排除标点符号。
这是为了让根据单词边界猜测事物变得更加困难。
所有文本都将编码为小写字母。

## Examples

## 示例

- Encoding `test` gives `gvhg`
- Encoding `x123 yes` gives `c123b vh`
- Decoding `gvhg` gives `test`
- Decoding `gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt` gives `thequickbrownfoxjumpsoverthelazydog`


- 编码 `test` 给出 `gvhg`
- 编码 `x123 yes` 给出 `c123b vh`
- 解码 `gvhg` 给出 `test`
- 解码 `gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt` 给出 `thequickbrownfoxjumpsoverthelazydog`