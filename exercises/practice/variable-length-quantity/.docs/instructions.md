# Instructions

# 指示

Implement variable length quantity encoding and decoding.

实现变长量编码和解码。

The goal of this exercise is to implement [VLQ][vlq] encoding/decoding.

本练习的目标是实现 [VLQ][vlq] 编码/解码。

In short, the goal of this encoding is to encode integer values in a way that would save bytes.
Only the first 7 bits of each byte are significant (right-justified; sort of like an ASCII byte).
So, if you have a 32-bit value, you have to unpack it into a series of 7-bit bytes.
Of course, you will have a variable number of bytes depending upon your integer.
To indicate which is the last byte of the series, you leave bit #7 clear.
In all of the preceding bytes, you set bit #7.

简而言之，这种编码的目标是以节省字节的方式对整数值进行编码。
只有每个字节的前 7 位是有效的（右对齐；有点像 ASCII 字节）。
因此，如果您有一个 32 位值，则必须将其解压缩为一系列 7 位字节。
当然，根据您的整数，您将拥有可变数量的字节。
要指示哪个是该系列的最后一个字节，请将位 #7 保留为清零。
在前面的所有字节中，您设置位#7。

So, if an integer is between `0-127`, it can be represented as one byte.
Although VLQ can deal with numbers of arbitrary sizes, for this exercise we will restrict ourselves to only numbers that fit in a 32-bit unsigned integer.
Here are examples of integers as 32-bit values, and the variable length quantities that they translate to:

所以，如果一个整数在`0-127`之间，它就可以表示为一个字节。
尽管 VLQ 可以处理任意大小的数字，但在本次练习中，我们将仅限于适合 32 位无符号整数的数字。
以下是 32 位值的整数示例，以及它们转换为的可变长度数量：

```text
 NUMBER        VARIABLE QUANTITY
00000000              00
00000040              40
0000007F              7F
00000080             81 00
00002000             C0 00
00003FFF             FF 7F
00004000           81 80 00
00100000           C0 80 00
001FFFFF           FF FF 7F
00200000          81 80 80 00
08000000          C0 80 80 00
0FFFFFFF          FF FF FF 7F
```

[vlq]: https://en.wikipedia.org/wiki/Variable-length_quantity


[vlq]：https://en.wikipedia.org/wiki/Variable-length_quantity