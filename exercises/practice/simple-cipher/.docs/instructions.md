# Instructions

# 指示

Create an implementation of the [Vigenère cipher][wiki].
The Vigenère cipher is a simple substitution cipher.

创建 [Vigenère 密码][wiki] 的实现。
维吉尼亚密码是一种简单的替换密码。

## Cipher terminology

## 密码术语

A cipher is an algorithm used to encrypt, or encode, a string.
The unencrypted string is called the _plaintext_ and the encrypted string is called the _ciphertext_.
Converting plaintext to ciphertext is called _encoding_ while the reverse is called _decoding_.

密码是一种用于加密或编码字符串的算法。
未加密的字符串称为_明文_，加密的字符串称为_密文_。
将明文转换为密文称为_编码_，而相反的过程称为_解码_。

In a _substitution cipher_, each plaintext letter is replaced with a ciphertext letter which is computed with the help of a _key_.
(Note, it is possible for replacement letter to be the same as the original letter.)

在_替换密码_中，每个明文字母都被替换为在_密钥_的帮助下计算出的密文字母。
（请注意，替换字母可能与原始字母相同。）

## Encoding details

## 编码细节

In this cipher, the key is a series of lowercase letters, such as `"abcd"`.
Each letter of the plaintext is _shifted_ or _rotated_ by a distance based on a corresponding letter in the key.
An `"a"` in the key means a shift of 0 (that is, no shift).
A `"b"` in the key means a shift of 1.
A `"c"` in the key means a shift of 2, and so on.

在此密码中，密钥是一系列小写字母，例如 `"abcd"`。
明文的每个字母都会根据密钥中对应的字母_移位_或_旋转_一定距离。
键中的 `"a"` 表示移位 0（即无移位）。
键中的 `"b"` 表示移位 1。
密钥中的 `"c"` 表示移位 2，依此类推。

The first letter of the plaintext uses the first letter of the key, the second letter of the plaintext uses the second letter of the key and so on.
If you run out of letters in the key before you run out of letters in the plaintext, start over from the start of the key again.

明文的第一个字母使用密钥的第一个字母，明文的第二个字母使用密钥的第二个字母，依此类推。
如果在用完明文中的字母之前用完密钥中的字母，请再次从密钥的开头重新开始。

If the key only contains one letter, such as `"dddddd"`, then all letters of the plaintext are shifted by the same amount (three in this example), which would make this the same as a rotational cipher or shift cipher (sometimes called a Caesar cipher).
For example, the plaintext `"iamapandabear"` would become `"ldpdsdqgdehdu"`.

如果密钥仅包含一个字母，例如 `"dddddd"`，则明文的所有字母都会移位相同的量（本例中为 3 个），这将使其与旋转密码或移位密码（有时称为凯撒密码）相同。
例如，明文 `"iamapandabear"` 将变为 `"ldpdsdqgdehdu"`。

If the key only contains the letter `"a"` (one or more times), the shift distance is zero and the ciphertext is the same as the plaintext.

如果密钥只包含字母`"a"`（一次或多次），则移位距离为零，密文与明文相同。

Usually the key is more complicated than that, though!
If the key is `"abcd"` then letters of the plaintext would be shifted by a distance of 0, 1, 2, and 3.
If the plaintext is `"hello"`, we need 5 shifts so the key would wrap around, giving shift distances of 0, 1, 2, 3, and 0.
Applying those shifts to the letters of `"hello"` we get `"hfnoo"`.

不过，通常密钥比这更复杂！
如果密钥是`"abcd"`，那么明文的字母将移动0、1、2和3的距离。
如果明文是 `"hello"`，我们需要 5 次移位，这样密钥就会环绕，移位距离为 0、1、2、3 和 0。
将这些移位应用于 `"hello"` 的字母，我们得到 `"hfnoo"`。

## Random keys

## 随机键

If no key is provided, generate a key which consists of at least 100 random lowercase letters from the Latin alphabet.

如果未提供密钥，则生成一个由至少 100 个拉丁字母中的随机小写字母组成的密钥。

[wiki]: https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher


[维基]：https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher