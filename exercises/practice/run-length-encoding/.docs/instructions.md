# Instructions

# 指示

Implement run-length encoding and decoding.

实现游程编码和解码。

Run-length encoding (RLE) is a simple form of data compression, where runs (consecutive data elements) are replaced by just one data value and count.

游程编码 (RLE) 是一种简单的数据压缩形式，其中游程（连续的数据元素）仅由一个数据值和计数替换。

For example we can represent the original 53 characters with only 13.

例如我们可以用13来表示原来的53个字符。

```text
"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB"  ->  "12WB12W3B24WB"
```

RLE allows the original data to be perfectly reconstructed from the compressed data, which makes it a lossless data compression.

RLE 允许从压缩数据完美地重建原始数据，这使其成为无损数据压缩。

```text
"AABCCCDEEEE"  ->  "2AB3CD4E"  ->  "AABCCCDEEEE"
```

For simplicity, you can assume that the unencoded string will only contain the letters A through Z (either lower or upper case) and whitespace.
This way data to be encoded will never contain any numbers and numbers inside data to be decoded always represent the count for the following character.


为简单起见，您可以假设未编码的字符串仅包含字母 A 到 Z（小写或大写）和空格。
这样，要编码的数据将永远不会包含任何数字，并且要解码的数据内部的数字始终表示后续字符的计数。