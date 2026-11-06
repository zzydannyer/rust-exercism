# Instructions

# 指示

Calculate the Hamming distance between two DNA strands.

计算两条 DNA 链之间的汉明距离。

We read DNA using the letters C, A, G and T.
Two strands might look like this:

我们使用字母 C、A、G 和 T 来读取 DNA。
两条线可能看起来像这样：

    GAGCCTACTAACGGGAT
    CATCGTAATGACGGCCT
    ^ ^ ^  ^ ^    ^^

They have 7 differences, and therefore the Hamming distance is 7.

它们有 7 个差异，因此汉明距离为 7。

## Implementation notes

## 实施说明

The Hamming distance is only defined for sequences of equal length, so an attempt to calculate it between sequences of different lengths should not work.


汉明距离仅针对相等长度的序列定义，因此尝试在不同长度的序列之间计算汉明距离是行不通的。