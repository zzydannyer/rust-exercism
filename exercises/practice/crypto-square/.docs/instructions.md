# Instructions

# 指示

Implement the classic method for composing secret messages called a square code.

实施称为方形代码的编写秘密消息的经典方法。

Given an English text, output the encoded version of that text.

给定英文文本，输出该文本的编码版本。

First, the input is normalized: the spaces and punctuation are removed from the English text and the message is down-cased.

首先，输入被标准化：从英文文本中删除空格和标点符号，并且消息被小写。

Then, the normalized characters are broken into rows.
These rows can be regarded as forming a rectangle when printed with intervening newlines.

然后，规范化的字符被分成行。
当打印中间有换行符时，这些行可以被视为形成一个矩形。

For example, the sentence

例如，句子

```text
"If man was meant to stay on the ground, god would have given us roots."
```

is normalized to:

标准化为：

```text
"ifmanwasmeanttostayonthegroundgodwouldhavegivenusroots"
```

The plaintext should be organized into a rectangle as square as possible.
The size of the rectangle should be decided by the length of the message.

明文应组织成尽可能正方形的矩形。
矩形的大小应由消息的长度决定。

If `c` is the number of columns and `r` is the number of rows, then for the rectangle `r` x `c` find the smallest possible integer `c` such that:

如果 `c` 是列数，`r` 是行数，则对于矩形 `r` x `c` 找到最小的可能整数 `c`，使得：

- `r * c >= length of message`,
- and `c >= r`,
- and `c - r <= 1`.

- `r * c >= length of message`，
- 和 `c >= r`，
- 和 `c - r <= 1`。

Our normalized text is 54 characters long, dictating a rectangle with `c = 8` and `r = 7`:

我们的规范化文本长度为 54 个字符，用 `c = 8` 和 `r = 7` 指定一个矩形：

```text
"ifmanwas"
"meanttos"
"tayonthe"
"groundgo"
"dwouldha"
"vegivenu"
"sroots  "
```

The coded message is obtained by reading down the columns going left to right.

通过从左到右向下阅读各列可以获得编码消息。

The message above is coded as:

上面的消息编码为：

```text
"imtgdvsfearwermayoogoanouuiontnnlvtwttddesaohghnsseoau"
```

Output the encoded text in chunks that fill perfect rectangles `(r X c)`, with `c` chunks of `r` length, separated by spaces.
For phrases that are `n` characters short of the perfect rectangle, pad each of the last `n` chunks with a single trailing space.

以填充完美矩形 `(r X c)` 的块形式输出编码文本，其中 `c` 长度为 `r` 块，并以空格分隔。
对于缺少完美矩形的 `n` 字符的短语，请使用单个尾随空格填充最后的每个 `n` 块。

```text
"imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau "
```

Notice that were we to stack these, we could visually decode the ciphertext back in to the original message:

请注意，如果我们将它们堆叠起来，我们可以直观地将密文解码回原始消息：

```text
"imtgdvs"
"fearwer"
"mayoogo"
"anouuio"
"ntnnlvt"
"wttddes"
"aohghn "
"sseoau "
```
