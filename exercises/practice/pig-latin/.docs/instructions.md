# Instructions

# 指示

Your task is to translate text from English to Pig Latin.
The translation is defined using four rules, which look at the pattern of vowels and consonants at the beginning of a word.
These rules look at each word's use of vowels and consonants:

您的任务是将文本从英语翻译成 Pig Latin。
翻译是使用四个规则来定义的，这些规则着眼于单词开头的元音和辅音的模式。
这些规则着眼于每个单词对元音和辅音的使用：

- vowels: the letters `a`, `e`, `i`, `o`, and `u`
- consonants: the other 21 letters of the English alphabet

- 元音：字母 `a`、`e`、`i`、`o` 和 `u`
- 辅音：英语字母表中的其他 21 个字母

## Rule 1

## 规则 1

If a word begins with a vowel, or starts with `"xr"` or `"yt"`, add an `"ay"` sound to the end of the word.

如果单词以元音开头，或者以 `"xr"` 或 `"yt"` 开头，请在单词末尾添加 `"ay"` 声音。

For example:

例如：

- `"apple"` -> `"appleay"` (starts with vowel)
- `"xray"` -> `"xrayay"` (starts with `"xr"`)
- `"yttria"` -> `"yttriaay"` (starts with `"yt"`)

- `"apple"` -> `"appleay"`（以元音开头）
- `"xray"` -> `"xrayay"`（以 `"xr"` 开头）
- `"yttria"` -> `"yttriaay"`（以 `"yt"` 开头）

## Rule 2

## 规则 2

If a word begins with one or more consonants, first move those consonants to the end of the word and then add an `"ay"` sound to the end of the word.

如果单词以一个或多个辅音开头，请首先将这些辅音移动到单词的末尾，然后将 `"ay"` 声音添加到单词的末尾。

For example:

例如：

- `"pig"` -> `"igp"` -> `"igpay"` (starts with single consonant)
- `"chair"` -> `"airch"` -> `"airchay"` (starts with multiple consonants)
- `"thrush"` -> `"ushthr"` -> `"ushthray"` (starts with multiple consonants)

- `"pig"` -> `"igp"` -> `"igpay"`（以单个辅音开头）
- `"chair"` -> `"airch"` -> `"airchay"`（以多个辅音开头）
- `"thrush"` -> `"ushthr"` -> `"ushthray"`（以多个辅音开头）

## Rule 3

## 规则 3

If a word starts with zero or more consonants followed by `"qu"`, first move those consonants (if any) and the `"qu"` part to the end of the word, and then add an `"ay"` sound to the end of the word.

如果单词以零个或多个辅音开头，后跟 `"qu"`，则首先将这些辅音（如果有）和 `"qu"` 部分移动到单词的末尾，然后将 `"ay"` 声音添加到单词的末尾。

For example:

例如：

- `"quick"` -> `"ickqu"` -> `"ickquay"` (starts with `"qu"`, no preceding consonants)
- `"square"` -> `"aresqu"` -> `"aresquay"` (starts with one consonant followed by `"qu`")

- `"quick"` -> `"ickqu"` -> `"ickquay"`（以 `"qu"` 开头，前面没有辅音）
- `"square"` -> `"aresqu"` -> `"aresquay"`（以一个辅音开头，后跟 `"qu`”）

## Rule 4

## 规则 4

If a word starts with one or more consonants followed by `"y"`, first move the consonants preceding the `"y"`to the end of the word, and then add an `"ay"` sound to the end of the word.

如果单词以一个或多个辅音开头，后跟 `"y"`，则首先将 `"y"` 前面的辅音移至单词末尾，然后将 `"ay"` 声音添加到单词末尾。

Some examples:

一些例子：

- `"my"` -> `"ym"` -> `"ymay"` (starts with single consonant followed by `"y"`)
- `"rhythm"` -> `"ythmrh"` -> `"ythmrhay"` (starts with multiple consonants followed by `"y"`)


- `"my"` -> `"ym"` -> `"ymay"`（以单个辅音开头，后跟 `"y"`）
- `"rhythm"` -> `"ythmrh"` -> `"ythmrhay"`（以多个辅音开头，后跟 `"y"`）