# Instructions

# 指示

Search files for lines matching a search string and return all matching lines.

在文件中搜索与搜索字符串匹配的行并返回所有匹配的行。

The Unix [`grep`][grep] command searches files for lines that match a regular expression.
Your task is to implement a simplified `grep` command, which supports searching for fixed strings.

Unix [`grep`][grep] 命令在文件中搜索与正则表达式匹配的行。
您的任务是实现一个简化的 `grep` 命令，该命令支持搜索固定字符串。

The `grep` command takes three arguments:

`grep` 命令采用三个参数：

1. The string to search for.
2. Zero or more flags for customizing the command's behavior.
3. One or more files to search in.

1. 要搜索的字符串。
2. 零个或多个用于自定义命令行为的标志。
3. 要搜索的一个或多个文件。

It then reads the contents of the specified files (in the order specified), finds the lines that contain the search string, and finally returns those lines in the order in which they were found.
When searching in multiple files, each matching line is prepended by the file name and a colon (':').

然后，它读取指定文件的内容（按指定的顺序），查找包含搜索字符串的行，最后按找到的顺序返回这些行。
在多个文件中搜索时，每个匹配行前面都会添加文件名和冒号（“:”）。

## Flags

## 旗帜

The `grep` command supports the following flags:

`grep` 命令支持以下标志：

- `-n` Prepend the line number and a colon (':') to each line in the output, placing the number after the filename (if present).
- `-l` Output only the names of the files that contain at least one matching line.
- `-i` Match using a case-insensitive comparison.
- `-v` Invert the program -- collect all lines that fail to match.
- `-x` Search only for lines where the search string matches the entire line.

- `-n` 在输出中的每一行前面添加行号和冒号（“：”），并将数字放在文件名（如果存在）之后。
- `-l` 只输出至少包含一个匹配行的文件名。
- `-i` 使用不区分大小写的比较进行匹配。
- `-v` 反转程序——收集所有无法匹配的行。
- `-x` 仅搜索搜索字符串与整行匹配的行。

[grep]: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html


[grep]：https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html