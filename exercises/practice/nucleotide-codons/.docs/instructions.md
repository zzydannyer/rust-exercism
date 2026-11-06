# Instructions

# 指示

Write a function that returns the name of an amino acid a particular codon,
possibly using shorthand, encodes for.

编写一个函数，返回特定密码子的氨基酸名称，
可能使用简写，编码为。

In DNA sequences of 3 nucleotides, called codons, encode for amino acids. Often
several codons encode for the same amino acid. The International Union of Pure
and Applied Chemistry developed a shorthand system for designating groups of
codons that encode for the same amino acid.

在 3 个核苷酸的 DNA 序列中，称为密码子，编码氨基酸。经常
多个密码子编码相同的氨基酸。国际纯净联盟
和应用化学开发了一个速记系统来指定组
编码相同氨基酸的密码子。

Simply put they've expanded the four letters A, C, G and T with a bunch of
letters that stand for different possibilities. For example R means A and G.
So TAR stands for TAA and TAG (think of "TAR" as "TA[AG]" in regex notation).

简单地说，他们用一堆扩展了 A、C、G 和 T 四个字母
字母代表不同的可能性。例如，R 表示 A 和 G。
因此，TAR 代表 TAA 和 TAG（将“TAR”视为正则表达式符号中的“TA[AG]”）。

Write some code that given a codon, which may use shorthand, returns the
name of the amino acid that that codon encodes for. You will be given
a list of non-shorthand-codon/name pairs to base your computation on.

编写一些代码，给定密码子（可以使用简写）返回
该密码子编码的氨基酸的名称。你将被给予
作为计算基础的非速记密码子/名称对的列表。

See: [wikipedia](https://en.wikipedia.org/wiki/DNA_codon_table).


请参阅：[wikipedia](https://en.wikipedia.org/wiki/DNA_codon_table)。