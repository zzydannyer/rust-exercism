# Instructions

# 指示

A [Domain Specific Language (DSL)][dsl] is a small language optimized for a specific domain.
Since a DSL is targeted, it can greatly impact productivity/understanding by allowing the writer to declare _what_ they want rather than _how_.

[领域特定语言 (DSL)][dsl] 是一种针对特定领域优化的小型语言。
由于 DSL 是有针对性的，因此它允许作者声明他们想要什么而不是如何想要，从而极大地影响生产力/理解。

One problem area where they are applied are complex customizations/configurations.

应用它们的一个问题领域是复杂的定制/配置。

For example the [DOT language][dot-language] allows you to write a textual description of a graph which is then transformed into a picture by one of the [Graphviz][graphviz] tools (such as `dot`).
A simple graph looks like this:

例如，[DOT 语言][点语言] 允许您编写图形的文本描述，然后通过 [Graphviz][graphviz] 工具之一（例如 `dot`）将其转换为图片。
一个简单的图表如下所示：

    graph {
        graph [bgcolor="yellow"]
        a [color="red"]
        b [color="blue"]
        a -- b [color="green"]
    }

Putting this in a file `example.dot` and running `dot example.dot -T png -o example.png` creates an image `example.png` with red and blue circle connected by a green line on a yellow background.

将其放入文件 `example.dot` 并运行 `dot example.dot -T png -o example.png` 会创建一个图像 `example.png`，其中红色和蓝色圆圈通过黄色背景上的绿线连接。

Write a Domain Specific Language similar to the Graphviz dot language.

编写类似于 Graphviz 点语言的领域特定语言。

Our DSL is similar to the Graphviz dot language in that our DSL will be used to create graph data structures.
However, unlike the DOT Language, our DSL will be an internal DSL for use only in our language.

我们的 DSL 与 Graphviz 点语言类似，因为我们的 DSL 将用于创建图形数据结构。
然而，与 DOT 语言不同，我们的 DSL 将是仅在我们的语言中使用的内部 DSL。

[Learn more about the difference between internal and external DSLs][fowler-dsl].

[详细了解内部 DSL 和外部 DSL 之间的区别][fowler-dsl]。

[dsl]: https://en.wikipedia.org/wiki/Domain-specific_language
[dot-language]: https://en.wikipedia.org/wiki/DOT_(graph_description_language)
[graphviz]: https://graphviz.org/
[fowler-dsl]: https://martinfowler.com/bliki/DomainSpecificLanguage.html


[dsl]：https://en.wikipedia.org/wiki/Domain-specific_language
[点语言]：https://en.wikipedia.org/wiki/DOT_(graph_description_language）
[图可视化]：https://graphviz.org/
[福勒-DSL]：https://martinfowler.com/bliki/DomainSpecificLanguage.html