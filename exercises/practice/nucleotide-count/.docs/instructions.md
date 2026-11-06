# Instructions

# 指示

Each of us inherits from our biological parents a set of chemical instructions known as DNA that influence how our bodies are constructed.
All known life depends on DNA!

我们每个人都从亲生父母那里继承了一套称为 DNA 的化学指令，它会影响我们身体的构造。
所有已知的生命都依赖于 DNA！

> Note: You do not need to understand anything about nucleotides or DNA to complete this exercise.

> 注意：您不需要了解任何有关核苷酸或 DNA 的知识即可完成此练习。

DNA is a long chain of other chemicals and the most important are the four nucleotides, adenine, cytosine, guanine and thymine.
A single DNA chain can contain billions of these four nucleotides and the order in which they occur is important!
We call the order of these nucleotides in a bit of DNA a "DNA sequence".

DNA 是由其他化学物质组成的长链，最重要的是四种核苷酸：腺嘌呤、胞嘧啶、鸟嘌呤和胸腺嘧啶。
一条 DNA 链可以包含数十亿个这四种核苷酸，它们出现的顺序很重要！
我们将一段 DNA 中这些核苷酸的顺序称为“DNA 序列”。

We represent a DNA sequence as an ordered collection of these four nucleotides and a common way to do that is with a string of characters such as "ATTACG" for a DNA sequence of 6 nucleotides.
'A' for adenine, 'C' for cytosine, 'G' for guanine, and 'T' for thymine.

我们将 DNA 序列表示为这四个核苷酸的有序集合，常见的方法是使用一串字符（例如“ATTACG”）来表示 6 个核苷酸的 DNA 序列。
“A”代表腺嘌呤，“C”代表胞嘧啶，“G”代表鸟嘌呤，“T”代表胸腺嘧啶。

Given a string representing a DNA sequence, count how many of each nucleotide is present.
If the string contains characters that aren't A, C, G, or T then it is invalid and you should signal an error.

给定一个代表 DNA 序列的字符串，计算每个核苷酸存在的数量。
如果字符串包含 A、C、G 或 T 以外的字符，则该字符串无效，您应该发出错误信号。

For example:

例如：

```text
"GATTACA" -> 'A': 3, 'C': 1, 'G': 1, 'T': 2
"INVALID" -> error
```
