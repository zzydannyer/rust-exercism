# Instructions

# 指示

Reparent a tree on a selected node.

在选定的节点上重新设置树的父级。

A [tree][wiki-tree] is a special type of [graph][wiki-graph] where all nodes are connected but there are no cycles.
That means, there is exactly one path to get from one node to another for any pair of nodes.

[树][维基树]是一种特殊类型的[图][维基图]，其中所有节点都是连接的，但没有循环。
这意味着，对于任何一对节点，都只有一条从一个节点到另一个节点的路径。

This exercise is all about re-orientating a tree to see things from a different point of view.
For example family trees are usually presented from the ancestor's perspective:

这个练习的目的是重新调整树的方向，以便从不同的角度看待事物。
例如，家谱通常是从祖先的角度呈现的：

```text
    +------0------+
    |      |      |
  +-1-+  +-2-+  +-3-+
  |   |  |   |  |   |
  4   5  6   7  8   9
```

But there is no inherent direction in a tree.
The same information can be presented from the perspective of any other node in the tree, by pulling it up to the root and dragging its relationships along with it.
So the same tree from 6's perspective would look like:

但树没有固有的方向。
通过将其拉至根并拖动其关系，可以从树中任何其他节点的角度呈现相同的信息。
所以从 6 的角度看，同一棵树看起来像：

```text
        6
        |
  +-----2-----+
  |           |
  7     +-----0-----+
        |           |
      +-1-+       +-3-+
      |   |       |   |
      4   5       8   9
```

This lets us more simply describe the paths between two nodes.
So for example the path from 6-9 (which in the first tree goes up to the root and then down to a different leaf node) can be seen to follow the path 6-2-0-3-9.

这让我们可以更简单地描述两个节点之间的路径。
例如，从 6-9 的路径（在第一棵树中向上到达根，然后向下到达不同的叶节点）可以被视为遵循路径 6-2-0-3-9。

This exercise involves taking an input tree and re-orientating it from the point of view of one of the nodes.

此练习涉及获取输入树并从其中一个节点的角度重新调整其方向。

[wiki-graph]: https://en.wikipedia.org/wiki/Tree_(graph_theory)
[wiki-tree]: https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)


[维基图]：https://en.wikipedia.org/wiki/Tree_(graph_theory）
[维基树]：https://en.wikipedia.org/wiki/Graph_(discrete_mathematics）