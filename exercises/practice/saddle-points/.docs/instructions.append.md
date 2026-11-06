# Instructions append

# 追加说明

## Rust Indices Start At 0

## Rust 指数从 0 开始

By convention, ordered sequences of values in Rust have their contents numbered
("indexed") starting from 0. This applies regardless of what the rest of the
exercise description in this README says, such as references to indices that
start at 1, so you will have to subtract 1 to translate those index numbers
to Rust index numbers.

按照惯例，Rust 中的有序值序列的内容是编号的
（“索引”）从 0 开始。无论其余部分如何，这都适用
本自述文件中的练习描述说，例如对索引的引用
从 1 开始，因此您必须减去 1 才能转换这些索引号
到 Rust 索引号。

## Efficiency Notice

## 效率通知

This exercise uses a _vector of vectors_ to store the content of matrices. While
this exercise is designed to help students understand basic concepts about
vectors, such as indexing, and that nested data types are legal, _vector of
vectors_ is a suboptimal choice for high-performance matrix algebra and any
similar efficient processing of larger amounts of data.

本练习使用_向量的向量_来存储矩阵的内容。同时
这个练习旨在帮助学生理解以下基本概念
向量，例如索引，并且嵌套数据类型是合法的，_vector 的
对于高性能矩阵代数和任何
对大量数据进行类似的有效处理。

The detailed explanation of this inefficiency is beyond the scope of this
exercise and this learning track in general. This aspect is known as
[cache locality](https://stackoverflow.com/questions/12065774/why-does-cache-locality-matter-for-array-performance)
and you can find a good introduction to it by clicking that link if you'd like
to learn more about details of a modern computer architecture.


这种低效率的详细解释超出了本文的范围
练习和一般的学习轨迹。这方面被称为
ZXQ保护0QXZ
如果您愿意，您可以通过单击该链接找到关于它的详细介绍
了解更多有关现代计算机体系结构的细节。