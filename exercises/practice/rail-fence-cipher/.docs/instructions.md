# Instructions

# 指示

Implement encoding and decoding for the rail fence cipher.

实现围栏密码的编码和解码。

The Rail Fence cipher is a form of transposition cipher that gets its name from the way in which it's encoded.
It was already used by the ancient Greeks.

铁栅栏密码是转置密码的一种形式，因其编码方式而得名。
它已经被古希腊人使用了。

In the Rail Fence cipher, the message is written downwards on successive "rails" of an imaginary fence, then moving up when we get to the bottom (like a zig-zag).
Finally the message is then read off in rows.

在铁栅栏密码中，消息被向下写在假想的栅栏的连续“铁轨”上，然后当我们到达底部时向上移动（就像之字形）。
最后，消息被逐行读出。

For example, using three "rails" and the message "WE ARE DISCOVERED FLEE AT ONCE", the cipherer writes out:

例如，使用三个“rails”和消息“WE ARE DISCOVERED FLEE AT ONCE”，密码器会写出：

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . A . . . I . . . V . . . D . . . E . . . N . .
```

Then reads off:

然后读出：

```text
WECRLTEERDSOEEFEAOCAIVDEN
```

To decrypt a message you take the zig-zag shape and fill the ciphertext along the rows.

要解密消息，您可以采用之字形并沿行填充密文。

```text
? . . . ? . . . ? . . . ? . . . ? . . . ? . . . ?
. ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
```

The first row has seven spots that can be filled with "WECRLTE".

第一行有七个可以填充“WECRLTE”的位置。

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
```

Now the 2nd row takes "ERDSOEEFEAOC".

现在第二行采用“ERDSOEEFEAOC”。

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
```

Leaving "AIVDEN" for the last row.

将“AIVDEN”留在最后一行。

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . A . . . I . . . V . . . D . . . E . . . N . .
```

If you now read along the zig-zag shape you can read the original message.


如果您现在沿着之字形阅读，您可以阅读原始消息。