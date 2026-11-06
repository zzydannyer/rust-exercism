# Instructions

# 指示

Your task is to count how many times each word occurs in a subtitle of a drama.

你的任务是计算戏剧字幕中每个单词出现的次数。

The subtitles from these dramas use only ASCII characters.

这些电视剧的字幕仅使用 ASCII 字符。

The characters often speak in casual English, using contractions like _they're_ or _it's_.
Though these contractions come from two words (e.g. _we are_), the contraction (_we're_) is considered a single word.

这些角色经常用随意的英语说话，使用诸如_they're_或_it's_之类的缩写。
尽管这些缩写来自两个单词（例如_we are_），但缩写（_we're_）被视为单个单词。

Words can be separated by any form of punctuation (e.g. ":", "!", or "?") or whitespace (e.g. "\t", "\n", or " ").
The only punctuation that does not separate words is the apostrophe in contractions.

单词可以用任何形式的标点符号（例如“:”、“!”或“?”）或空格（例如“\t”、“\n”或“”）分隔。
唯一不分隔单词的标点符号是缩写中的撇号。

Numbers are considered words.
If the subtitles say _It costs 100 dollars._ then _100_ will be its own word.

数字被视为单词。
如果字幕说“花费 100 美元”，那么 _100_ 将是它自己的单词。

Words are case insensitive.
For example, the word _you_ occurs three times in the following sentence:

单词不区分大小写。
例如，单词 _you_ 在以下句子中出现了 3 次：

> You come back, you hear me? DO YOU HEAR ME?

> 你回来吧，听到了吗？你听到我说话了吗？

The ordering of the word counts in the results doesn't matter.

结果中字数的顺序并不重要。

Here's an example that incorporates several of the elements discussed above:

这是一个包含上面讨论的几个元素的示例：

- simple words
- contractions
- numbers
- case insensitive words
- punctuation (including apostrophes) to separate words
- different forms of whitespace to separate words

- 简单的话
- 宫缩
- 数字
- 不区分大小写的单词
- 标点符号（包括撇号）来分隔单词
- 不同形式的空格来分隔单词

`"That's the password: 'PASSWORD 123'!", cried the Special Agent.\nSo I fled.`

ZXQ保护0QXZ

The mapping for this subtitle would be:

该字幕的映射为：

```text
123: 1
agent: 1
cried: 1
fled: 1
i: 1
password: 2
so: 1
special: 1
that's: 1
the: 2
```
