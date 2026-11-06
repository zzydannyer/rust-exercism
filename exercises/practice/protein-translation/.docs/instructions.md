# Instructions

# 指示

Your job is to translate RNA sequences into proteins.

你的工作是将 RNA 序列翻译成蛋白质。

RNA strands are made up of three-nucleotide sequences called **codons**.
Each codon translates to an **amino acid**.
When joined together, those amino acids make a protein.

RNA 链由称为 **密码子** 的三核苷酸序列组成。
每个密码子都会翻译成一个**氨基酸**。
当这些氨基酸结合在一起时，就形成了蛋白质。

In the real world, there are 64 codons, which in turn correspond to 20 amino acids.
However, for this exercise, you’ll only use a few of the possible 64.
They are listed below:

在现实世界中，有 64 个密码子，依次对应 20 个氨基酸。
但是，在本练习中，您将仅使用可能的 64 个中的几个。
下面列出了它们：

| Codon              | Amino Acid    |
| ------------------ | ------------- |
| AUG                | Methionine    |
| UUU, UUC           | Phenylalanine |
| UUA, UUG           | Leucine       |
| UCU, UCC, UCA, UCG | Serine        |
| UAU, UAC           | Tyrosine      |
| UGU, UGC           | Cysteine      |
| UGG                | Tryptophan    |
| UAA, UAG, UGA      | STOP          |

For example, the RNA string “AUGUUUUCU” has three codons: “AUG”, “UUU” and “UCU”.
These map to Methionine, Phenylalanine, and Serine.

例如，RNA 字符串“AUGUUUUCU”具有三个密码子：“AUG”、“UUU”和“UCU”。
这些映射到蛋氨酸、苯丙氨酸和丝氨酸。

## “STOP” Codons

## “STOP”密码子

You’ll note from the table above that there are three **“STOP” codons**.
If you encounter any of these codons, ignore the rest of the sequence — the protein is complete.

您会从上表中注意到，有三个**“STOP”密码子**。
如果您遇到任何这些密码子，请忽略序列的其余部分 - 蛋白质是完整的。

For example, “AUGUUUUCUUAAAUG” contains a STOP codon (“UAA”).
Once we reach that point, we stop processing.
We therefore only consider the part before it (i.e. “AUGUUUUCU”), not any further codons after it (i.e. “AUG”).

例如，“AUGUUUUCUUAAAUG”包含终止密码子（“UAA”）。
一旦达到这一点，我们就会停止处理。
因此，我们只考虑其之前的部分（即“AUGUUUUCU”），而不考虑其之后的任何其他密码子（即“AUG”）。

Learn more about [protein translation on Wikipedia][protein-translation].

了解有关[维基百科上的蛋白质翻译][蛋白质翻译]的更多信息。

[protein-translation]: https://en.wikipedia.org/wiki/Translation_(biology)


[蛋白质翻译]：https://en.wikipedia.org/wiki/Translation_(biology）